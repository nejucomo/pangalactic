use crate::options::{Destination, Source};
use pangalactic_dagio::{Dagio, ReadCommitter};
use pangalactic_layer_cidmeta::CidMetaLayer;
use pangalactic_store::Store;
use pangalactic_store_dirdb::DirDbStore;
use std::path::Path;
use tokio::io::{AsyncRead, AsyncWrite};

pub type StoreBackend = DirDbStore;
pub type Cid = <StoreBackend as Store>::CID;
pub type Link = pangalactic_dagio::DagioLink<StoreBackend>;
pub type Reader = pangalactic_dagio::DagioReader<StoreBackend>;
pub type StoreDestination = pangalactic_storepath::StoreDestination<CidMetaLayer<StoreBackend>>;
pub type StorePath = pangalactic_storepath::StorePath<CidMetaLayer<StoreBackend>>;
pub type HostDirectory = pangalactic_dagio::DagioHostDirectory<StoreBackend>;

#[derive(Debug, Default)]
pub struct Commander {
    dagio: Dagio<StoreBackend>,
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
                    self.xfer_from_host_dir(p, dest).await
                }
            }
            Store(sp) => {
                use pangalactic_linkkind::LinkKind::*;

                match sp.kind() {
                    File => {
                        let r: Reader = self.dagio.load(sp.link()).await?;
                        self.xfer_from_stream(r, dest).await
                    }
                    Dir => self.xfer_from_store_dir(sp, dest).await,
                }
            }
        }
    }

    async fn xfer_from_stream<R>(&mut self, r: R, dest: &Destination) -> anyhow::Result<()>
    where
        R: AsyncRead + Send,
    {
        use Destination::*;

        match dest {
            Stdout => copy(r, tokio::io::stdout()).await,
            Host(p) => {
                let f = tokio::fs::File::create(p).await?;
                copy(r, f).await
            }
            Store(osd) => {
                let link = self
                    .dagio
                    .commit_to_opt(osd.as_ref(), ReadCommitter(r))
                    .await?;
                println!("{link}");
                Ok(())
            }
        }
    }

    async fn xfer_from_host_dir(
        &mut self,
        source: &Path,
        dest: &Destination,
    ) -> anyhow::Result<()> {
        use Destination::*;

        match dest {
            Stdout => anyhow::bail!("cannot xfer host dir {:?} to stdout", source.display()),
            Host(p) => xfer_from_host_dir_to_host(source, p).await,
            Store(osd) => {
                let link = self.dagio.commit_to_opt(osd.as_ref(), source).await?;
                println!("{link}");
                Ok(())
            }
        }
    }

    async fn xfer_from_store_dir(
        &mut self,
        source: &StorePath,
        dest: &Destination,
    ) -> anyhow::Result<()> {
        use Destination::*;

        match dest {
            Stdout => anyhow::bail!("cannot xfer store dir {source:?} to stdout"),
            Host(p) => self.xfer_from_store_dir_to_host(source, p).await,
            Store(osd) => {
                let link = self.dagio.commit_to_opt(osd.as_ref(), source).await?;
                println!("{link}");
                Ok(())
            }
        }
    }

    async fn xfer_from_store_dir_to_host(
        &mut self,
        source: &StorePath,
        dest: &Path,
    ) -> anyhow::Result<()> {
        use pangalactic_linkkind::LinkKind::File;
        use std::collections::VecDeque;

        let hd: HostDirectory = self.dagio.load_from(source).await?;
        let mut q = VecDeque::from([(hd, dest.to_path_buf())]);
        while let Some((source, dest)) = q.pop_front() {
            for (name, link) in source {
                let subdest = dest.join(name);
                if link.kind() == File {
                    let r: Reader = self.dagio.load(&link).await?;
                    let f = tokio::fs::File::create(subdest).await?;
                    copy(r, f).await?;
                } else {
                    tokio::fs::create_dir(&subdest).await?;
                    let hd: HostDirectory = self.dagio.load(&link).await?;
                    q.push_back((hd, subdest));
                }
            }
        }
        Ok(())
    }
}

async fn copy<R, W>(r: R, w: W) -> anyhow::Result<()>
where
    R: AsyncRead,
    W: AsyncWrite,
{
    let mut pinr = std::pin::pin!(r);
    let mut pinw = std::pin::pin!(w);
    tokio::io::copy(&mut pinr, &mut pinw).await?;
    Ok(())
}

async fn xfer_from_host_dir_to_host(source: &Path, dest: &Path) -> anyhow::Result<()> {
    use std::collections::VecDeque;

    let mut q = VecDeque::from([(source.to_path_buf(), dest.to_path_buf())]);
    while let Some((source, dest)) = q.pop_front() {
        let mut rdir = tokio::fs::read_dir(&source).await?;
        while let Some(entry) = rdir.next_entry().await? {
            let subdest = dest.join(entry.file_name());
            let subsource = entry.path();
            if subsource.is_file() {
                tokio::fs::copy(subsource, subdest).await?;
            } else {
                tokio::fs::create_dir(&subdest).await?;
                q.push_back((subsource, subdest));
            }
        }
    }
    Ok(())
}
