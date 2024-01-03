use pangalactic_dagio::{Dagio, DagioLink, DagioReader};
use pangalactic_layer_cidmeta::CidMetaLayer;
use pangalactic_store::Store;
use pangalactic_store_dirdb::DirDbStore;
use tokio::io::AsyncRead;

use crate::options::{Destination, Source};

pub type Cid = <DirDbStore as Store>::CID;
pub type Link = DagioLink<DirDbStore>;
pub type Reader = DagioReader<DirDbStore>;
pub type StoreDestination = pangalactic_storepath::StoreDestination<CidMetaLayer<DirDbStore>>;
pub type StorePath = pangalactic_storepath::StorePath<CidMetaLayer<DirDbStore>>;

#[derive(Debug, Default)]
pub struct Commander {
    dagio: Dagio<DirDbStore>,
}

impl Commander {
    pub async fn store_put(&mut self) -> anyhow::Result<()> {
        let mut r = tokio::io::stdin();
        let mut w = self.dagio.open_file_writer().await?;
        tokio::io::copy(&mut r, &mut w).await?;
        let link = self.dagio.commit(w).await?;
        println!("{link}");
        Ok(())
    }

    pub async fn store_get(&mut self, link: &Link) -> anyhow::Result<()> {
        let mut r: Reader = self.dagio.load(link).await?;
        let mut w = tokio::io::stdout();
        tokio::io::copy(&mut r, &mut w).await?;
        Ok(())
    }

    pub async fn store_xfer(&mut self, source: &Source, dest: &Destination) -> anyhow::Result<()> {
        use Source::*;

        match source {
            Stdin => self.xfer_from_stream(tokio::io::stdin(), dest).await,
            Host(p) => {
                if p.is_file() {
                    let f = tokio::fs::File::open(p).await?;
                    self.xfer_from_stream(f, dest).await
                } else {
                    todo!("from host dir")
                }
            }
            Store(sp) => {
                use pangalactic_linkkind::LinkKind::*;

                match sp.kind() {
                    File => {
                        let r: Reader = self.dagio.load(sp.link()).await?;
                        self.xfer_from_stream(r, dest).await
                    }
                    Dir => todo!("from store dir"),
                }
            }
        }
    }

    async fn xfer_from_stream<R>(&mut self, _r: R, _dest: &Destination) -> anyhow::Result<()>
    where
        R: AsyncRead,
    {
        todo!();
    }
}
