use std::{fmt::Debug, path::Path};

use tokio::{
    fs::File,
    io::{self, AsyncRead},
};

use crate::{
    options::{Destination, Source},
    store::{CliDagio, CliLink, CliStoreDirectory, CliStorePath},
};

#[derive(Debug, Default)]
pub struct StoreCommander(CliDagio);

impl StoreCommander {
    pub async fn put(&mut self) -> anyhow::Result<CliLink> {
        let link = self
            .xfer(&Source::Stdin, &Destination::StoreScheme)
            .await?
            .unwrap();
        Ok(link)
    }

    pub async fn get(&mut self, link: &CliLink) -> anyhow::Result<()> {
        let src = CliStorePath::new(link.clone(), vec![])?;
        let none = self.xfer(&Source::Store(src), &Destination::Stdout).await?;
        assert!(none.is_none());
        Ok(())
    }

    pub async fn xfer(
        &mut self,
        source: &Source,
        dest: &Destination,
    ) -> anyhow::Result<Option<CliLink>> {
        match source {
            Source::Stdin => self.xfer_from_stream(io::stdin(), dest).await,
            Source::Host(hostpath) => {
                if hostpath.is_file() {
                    let r = File::open(hostpath).await?;
                    self.xfer_from_stream(r, dest).await
                } else if hostpath.is_dir() {
                    self.xfer_from_hostdir(hostpath, dest).await
                } else {
                    anyhow::bail!("Unknown fs node type: {:?}", hostpath.display())
                }
            }
            Source::Store(storepath) => {
                use pangalactic_dagio::DagioReadNode::*;

                let readnode = self.0.load(storepath).await?;
                match readnode {
                    FileReader(r) => self.xfer_from_stream(r, dest).await,
                    Dir(d) => self.xfer_from_storedir(d, dest).await,
                }
            }
        }
    }

    async fn xfer_from_stream<R>(
        &mut self,
        srcstream: R,
        dest: &Destination,
    ) -> anyhow::Result<Option<CliLink>>
    where
        R: AsyncRead + Debug,
    {
        use std::pin::pin;
        use Destination::*;

        let mut ssp = pin!(srcstream);

        match dest {
            Stdout => {
                io::copy(&mut ssp, &mut io::stdout()).await?;
                Ok(None)
            }
            Host(path) => {
                let mut w = File::create_new(path).await?;
                io::copy(&mut ssp, &mut w).await?;
                Ok(None)
            }
            StoreScheme => {
                let mut w = self.0.open_file_writer().await?;
                io::copy(&mut ssp, &mut w).await?;
                let link = self.0.commit(w).await?;
                Ok(Some(link))
            }
            Store(dest) => {
                todo!("{dest:?}");
            }
        }
    }

    async fn xfer_from_hostdir(
        &mut self,
        srcdir: &Path,
        dest: &Destination,
    ) -> anyhow::Result<Option<CliLink>> {
        dbg!(srcdir, dest);
        todo!()
    }

    async fn xfer_from_storedir(
        &mut self,
        srcdir: CliStoreDirectory,
        dest: &Destination,
    ) -> anyhow::Result<Option<CliLink>> {
        dbg!(srcdir, dest);
        todo!()
    }
}
