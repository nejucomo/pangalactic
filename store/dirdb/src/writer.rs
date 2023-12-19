use pangalactic_hash::{Hash, Hasher};
use pin_project::pin_project;
use std::path::{Path, PathBuf};
use std::pin::Pin;
use std::task::{Context, Poll};
use tokio::io::AsyncWrite;

#[pin_project]
#[derive(Debug)]
pub struct Writer {
    spoolpath: PathBuf,
    #[pin]
    f: tokio::fs::File,
    hasher: Hasher,
}

impl Writer {
    pub(crate) async fn init(dir: &Path) -> anyhow::Result<Self> {
        let spoolpath = dir.join(get_spool_name());
        let f = tokio::fs::File::create(&spoolpath).await?;
        let hasher = Hasher::default();

        Ok(Writer {
            f,
            spoolpath,
            hasher,
        })
    }

    pub(crate) async fn commit(self) -> anyhow::Result<Hash> {
        use anyhow_std::PathAnyhow;

        let Writer {
            spoolpath,
            f,
            hasher,
        } = self;

        f.sync_all().await?;
        let hash = hasher.finalize();

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
        use Poll::Ready;

        let this = self.project();
        match this.f.poll_write(cx, buf) {
            Ready(Ok(cnt)) => {
                use std::io::Write;

                this.hasher.write_all(&buf[..cnt])?;

                Ready(Ok(cnt))
            }

            other => other,
        }
    }

    fn poll_flush(self: Pin<&mut Self>, cx: &mut Context<'_>) -> Poll<Result<(), std::io::Error>> {
        self.project().f.poll_flush(cx)
    }

    fn poll_shutdown(
        self: Pin<&mut Self>,
        cx: &mut Context<'_>,
    ) -> Poll<Result<(), std::io::Error>> {
        self.project().f.poll_shutdown(cx)
    }
}

fn get_spool_name() -> String {
    use rand::distributions::Standard;
    use rand::Rng;

    let mut rng = rand::thread_rng();
    let r: [u8; 32] = rng.sample(Standard);
    let mut spoolname = "incoming.".to_string();
    pangalactic_b64::encode_to_string(r, &mut spoolname);

    spoolname
}
