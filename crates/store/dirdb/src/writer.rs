use std::path::{Path, PathBuf};
use std::pin::Pin;
use std::task::{Context, Poll};

use anyhow_std::PathAnyhow as _;
use pangalactic_hash::{Hash, HashWriter};
use pin_project::pin_project;
use tokio::io::AsyncWrite;

#[pin_project]
#[derive(Debug)]
pub struct Writer {
    spoolpath: PathBuf,
    #[pin]
    downstream: HashWriter<tokio::fs::File>,
}

impl Writer {
    pub(crate) async fn init(dir: &Path) -> anyhow::Result<Self> {
        dir.create_dir_all_anyhow()?;
        let spoolpath = dir.join(get_spool_name());
        let f = tokio::fs::File::create(&spoolpath).await?;
        let downstream = HashWriter::from(f);

        Ok(Writer {
            spoolpath,
            downstream,
        })
    }

    pub(crate) async fn commit(self) -> anyhow::Result<Hash> {
        use anyhow_std::PathAnyhow;

        let Writer {
            spoolpath,
            downstream,
        } = self;

        let (f, hash) = downstream.unwrap();
        f.sync_all().await?;

        // Change completed spool file to read-only:
        let mut perms = f.metadata().await?.permissions();
        perms.set_readonly(true);
        tokio::fs::set_permissions(&spoolpath, perms).await?;

        let dir = spoolpath.parent_anyhow()?;
        let destpath = dir.join(hash.to_string());

        // If dest is overwritten atomically, both copies should be identical, so there is no problem other than performance issues.
        spoolpath.rename_anyhow(destpath)?;

        Ok(hash)
    }
}

impl AsyncWrite for Writer {
    fn poll_write(
        self: Pin<&mut Self>,
        cx: &mut Context<'_>,
        buf: &[u8],
    ) -> Poll<Result<usize, std::io::Error>> {
        self.project().downstream.poll_write(cx, buf)
    }

    fn poll_flush(self: Pin<&mut Self>, cx: &mut Context<'_>) -> Poll<Result<(), std::io::Error>> {
        self.project().downstream.poll_flush(cx)
    }

    fn poll_shutdown(
        self: Pin<&mut Self>,
        cx: &mut Context<'_>,
    ) -> Poll<Result<(), std::io::Error>> {
        self.project().downstream.poll_shutdown(cx)
    }
}

fn get_spool_name() -> String {
    use rand::distr::StandardUniform;
    use rand::Rng;

    let mut rng = rand::rng();
    let r: [u8; 32] = rng.sample(StandardUniform);
    let mut spoolname = "incoming.".to_string();
    pangalactic_b64::encode_to_string(r, &mut spoolname);

    spoolname
}
