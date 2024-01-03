use pangalactic_dagio::{Dagio, DagioLink, DagioReader};
use pangalactic_store::Store;
use pangalactic_store_dirdb::DirDbStore;

pub type Cid = <DirDbStore as Store>::CID;
pub type Link = DagioLink<DirDbStore>;
pub type Reader = DagioReader<DirDbStore>;
pub type StoreDestination = pangalactic_storepath::StoreDestination<DirDbStore>;
pub type StorePath = pangalactic_storepath::StorePath<DirDbStore>;

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
}
