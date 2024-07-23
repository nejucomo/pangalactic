use std::{fmt::Debug, path::Path};

use tokio::io::{self, AsyncRead};

use crate::{
    options::{Destination, Source},
    store::{CliDagio, CliLink, CliReadNode, CliStoreDirectory, CliStorePath},
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
                    let r = tokio::fs::File::open(hostpath).await?;
                    self.xfer_from_stream(r, dest).await
                } else if hostpath.is_dir() {
                    self.xfer_from_hostdir(hostpath, dest).await
                } else {
                    anyhow::bail!("Unknown fs node type: {:?}", hostpath.display())
                }
            }
            Source::Store(storepath) => {
                let link = self.resolve_storepath(storepath).await?;
                let readnode: CliReadNode = self.0.load(&link).await?;
                match readnode {
                    CliReadNode::FileReader(r) => self.xfer_from_stream(r, dest).await,
                    CliReadNode::Dir(d) => self.xfer_from_storedir(d, dest).await,
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
        dbg!(srcstream, dest);
        todo!()
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

    async fn resolve_storepath(&self, storepath: &CliStorePath) -> anyhow::Result<CliLink> {
        dbg!(storepath);
        todo!();
    }
}
