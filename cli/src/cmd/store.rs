mod xfer;

use std::fmt::Debug;

use crate::{
    options::{Destination, Source},
    store::{CliDagio, CliLink, CliStorePath},
};

use self::xfer::XferInto;

#[derive(Debug, Default)]
pub struct StoreCommander(CliDagio);

impl StoreCommander {
    pub async fn put(&mut self) -> anyhow::Result<CliLink> {
        let link = self
            .xfer(&Source::Stdin, &Destination::Store(None))
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
        source.xfer_into(&mut self.0, dest).await
    }

    /*
    async fn xfer_from_hostdir(
        &mut self,
        srcdir: &Path,
        dest: &Destination,
    ) -> anyhow::Result<Option<CliLink>> {
        use Destination::*;

        match dest {
            Stdout => {
                anyhow::bail!("cannot xfer hostdir {:?} to stdout", srcdir.display());
            }
            Host(destdir) => copy_host_to_host(srcdir, destdir).await.map(|()| None),
            Store(optdest) => self.0.commit_into(srcdir, optdest.as_ref()).await.map(Some),
        }
    }

    async fn xfer_from_storedir(
        &mut self,
        srcdir: CliStoreDirectory,
        dest: &Destination,
    ) -> anyhow::Result<Option<CliLink>> {
        dbg!(srcdir, dest);
        todo!()
    }
    */
}
