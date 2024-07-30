mod xfer;

use std::fmt::Debug;

use pangalactic_dagio::Dagio;
use pangalactic_layer_cidmeta::CidMeta;
use pangalactic_link::Link;
use pangalactic_store::Store;
use pangalactic_store_dirdb::DirDbStore;
use pangalactic_storepath::StorePath;

use crate::options::{Destination, Source};

use self::xfer::XferInto;

#[derive(Debug, Default)]
pub struct StoreCommander(Dagio<DirDbStore>);

impl StoreCommander {
    pub async fn put(&mut self) -> anyhow::Result<Link<CidMeta<<DirDbStore as Store>::CID>>> {
        let link = self
            .xfer(&Source::Stdin, &Destination::Store(None))
            .await?
            .unwrap();
        Ok(link)
    }

    pub async fn get(
        &mut self,
        link: &Link<CidMeta<<DirDbStore as Store>::CID>>,
    ) -> anyhow::Result<()> {
        let src = StorePath::<CidMeta<<DirDbStore as Store>::CID>>::new(link.clone(), vec![])?;
        let none = self.xfer(&Source::Store(src), &Destination::Stdout).await?;
        assert!(none.is_none());
        Ok(())
    }

    pub async fn xfer(
        &mut self,
        source: &Source,
        dest: &Destination,
    ) -> anyhow::Result<Option<Link<CidMeta<<DirDbStore as Store>::CID>>>> {
        source.xfer_into(&mut self.0, dest).await
    }

    /*
    async fn xfer_from_hostdir(
        &mut self,
        srcdir: &Path,
        dest: &Destination,
    ) -> anyhow::Result<Option<Link<CidMeta<<DirDbStore as Store>::CID>>>> {
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
    ) -> anyhow::Result<Option<Link<CidMeta<<DirDbStore as Store>::CID>>>> {
        dbg!(srcdir, dest);
        todo!()
    }
    */
}
