use std::path::PathBuf;
use std::{path::Path, pin::pin};

use async_trait::async_trait;
use pangalactic_dagio::{Dagio, DagioReadCommitter, DagioReadNode, DagioReader};
use pangalactic_hostdir::HostDirectory;
use pangalactic_layer_cidmeta::CidMeta;
use pangalactic_link::Link;
use pangalactic_store::Store;
use pangalactic_store_dirdb::DirDbStore;
use pangalactic_path::{StoreDestination, StorePath};
use tokio::fs;
use tokio::io::{self, AsyncRead};

use crate::options::{Destination, Source};

#[cfg_attr(not(doc), async_trait)]
pub(super) trait XferInto: Sized {
    async fn xfer_into(
        self,
        dagio: &mut Dagio<DirDbStore>,
        dest: &Destination,
    ) -> anyhow::Result<Option<Link<CidMeta<<DirDbStore as Store>::CID>>>>;
}

#[cfg_attr(not(doc), async_trait)]
trait XferIntoParts: Sized + Send {
    async fn xfer_into_stdout(self, dagio: &mut Dagio<DirDbStore>) -> anyhow::Result<()>;

    async fn xfer_into_host(self, dagio: &mut Dagio<DirDbStore>, path: &Path)
        -> anyhow::Result<()>;

    async fn xfer_into_store(
        self,
        dagio: &mut Dagio<DirDbStore>,
        dest: Option<&StoreDestination<CidMeta<<DirDbStore as Store>::CID>>>,
    ) -> anyhow::Result<Link<CidMeta<<DirDbStore as Store>::CID>>>;
}

#[cfg_attr(not(doc), async_trait)]
impl<P> XferInto for P
where
    P: XferIntoParts,
{
    async fn xfer_into(
        self,
        dagio: &mut Dagio<DirDbStore>,
        dest: &Destination,
    ) -> anyhow::Result<Option<Link<CidMeta<<DirDbStore as Store>::CID>>>> {
        match dest {
            Destination::Stdout => self.xfer_into_stdout(dagio).await.map(|()| None),
            Destination::Host(hostpath) => {
                self.xfer_into_host(dagio, hostpath).await.map(|()| None)
            }
            Destination::Store(storepath) => self
                .xfer_into_store(dagio, storepath.as_ref())
                .await
                .map(Some),
        }
    }
}

#[cfg_attr(not(doc), async_trait)]
impl<'a> XferInto for &'a Source {
    async fn xfer_into(
        self,
        dagio: &mut Dagio<DirDbStore>,
        dest: &Destination,
    ) -> anyhow::Result<Option<Link<CidMeta<<DirDbStore as Store>::CID>>>> {
        match self {
            Source::Stdin => DagioReadCommitter(io::stdin()).xfer_into(dagio, dest).await,
            Source::Host(hostpath) => hostpath.as_path().xfer_into(dagio, dest).await,
            Source::Store(storepath) => storepath.xfer_into(dagio, dest).await,
        }
    }
}

#[cfg_attr(not(doc), async_trait)]
impl<R> XferIntoParts for DagioReadCommitter<R>
where
    R: AsyncRead + Send,
{
    async fn xfer_into_stdout(self, _: &mut Dagio<DirDbStore>) -> anyhow::Result<()> {
        io::copy(&mut pin!(self.0), &mut io::stdout()).await?;
        Ok(())
    }

    async fn xfer_into_host(self, _: &mut Dagio<DirDbStore>, path: &Path) -> anyhow::Result<()> {
        let mut f = fs::File::create_new(path).await?;
        io::copy(&mut pin!(self.0), &mut f).await?;
        Ok(())
    }

    async fn xfer_into_store(
        self,
        dagio: &mut Dagio<DirDbStore>,
        dest: Option<&StoreDestination<CidMeta<<DirDbStore as Store>::CID>>>,
    ) -> anyhow::Result<Link<CidMeta<<DirDbStore as Store>::CID>>> {
        dagio.commit_into(self, dest).await
    }
}

#[cfg_attr(not(doc), async_trait)]
impl XferInto for PathBuf {
    async fn xfer_into(
        self,
        dagio: &mut Dagio<DirDbStore>,
        dest: &Destination,
    ) -> anyhow::Result<Option<Link<CidMeta<<DirDbStore as Store>::CID>>>> {
        self.as_path().xfer_into(dagio, dest).await
    }
}

#[cfg_attr(not(doc), async_trait)]
impl<'a> XferInto for &'a Path {
    async fn xfer_into(
        self,
        dagio: &mut Dagio<DirDbStore>,
        dest: &Destination,
    ) -> anyhow::Result<Option<Link<CidMeta<<DirDbStore as Store>::CID>>>> {
        if self.is_file() {
            let f = fs::File::open(self).await?;
            DagioReadCommitter(f).xfer_into(dagio, dest).await
        } else if self.is_dir() {
            let rd = fs::read_dir(self).await?;
            rd.xfer_into(dagio, dest).await
        } else {
            anyhow::bail!(
                "cannot xfer from unknown host fs node type: {:?}",
                self.display()
            );
        }
    }
}

#[cfg_attr(not(doc), async_trait)]
impl XferIntoParts for fs::ReadDir {
    async fn xfer_into_stdout(self, _: &mut Dagio<DirDbStore>) -> anyhow::Result<()> {
        anyhow::bail!("cannot xfer host dir into stdout");
    }

    async fn xfer_into_host(
        mut self,
        dagio: &mut Dagio<DirDbStore>,
        path: &Path,
    ) -> anyhow::Result<()> {
        fs::create_dir(path).await?;

        while let Some(direntry) = self.next_entry().await? {
            let name = direntry.file_name();
            let dst = path.join(name);
            direntry
                .path()
                .xfer_into(dagio, &Destination::Host(dst))
                .await?;
        }

        Ok(())
    }

    async fn xfer_into_store(
        self,
        dagio: &mut Dagio<DirDbStore>,
        dest: Option<&StoreDestination<CidMeta<<DirDbStore as Store>::CID>>>,
    ) -> anyhow::Result<Link<CidMeta<<DirDbStore as Store>::CID>>> {
        dagio.commit_into(self, dest).await
    }
}

#[cfg_attr(not(doc), async_trait)]
impl<'a> XferInto for &'a StorePath<CidMeta<<DirDbStore as Store>::CID>> {
    async fn xfer_into(
        self,
        dagio: &mut Dagio<DirDbStore>,
        dest: &Destination,
    ) -> anyhow::Result<Option<Link<CidMeta<<DirDbStore as Store>::CID>>>> {
        use pangalactic_dagio::DagioResolveLink;

        let link = self.resolve_link(dagio).await?;
        link.xfer_into(dagio, dest).await
    }
}

#[cfg_attr(not(doc), async_trait)]
impl XferInto for Link<CidMeta<<DirDbStore as Store>::CID>> {
    async fn xfer_into(
        self,
        dagio: &mut Dagio<DirDbStore>,
        dest: &Destination,
    ) -> anyhow::Result<Option<Link<CidMeta<<DirDbStore as Store>::CID>>>> {
        let rnode: DagioReadNode<DirDbStore> = dagio.load(self).await?;
        match rnode {
            DagioReadNode::FileReader(r) => r.xfer_into(dagio, dest).await,
            DagioReadNode::Dir(hd) => hd.xfer_into(dagio, dest).await,
        }
    }
}

#[cfg_attr(not(doc), async_trait)]
impl XferInto for DagioReader<DirDbStore> {
    async fn xfer_into(
        self,
        dagio: &mut Dagio<DirDbStore>,
        dest: &Destination,
    ) -> anyhow::Result<Option<Link<CidMeta<<DirDbStore as Store>::CID>>>> {
        DagioReadCommitter(self).xfer_into(dagio, dest).await
    }
}

#[cfg_attr(not(doc), async_trait)]
impl XferIntoParts for HostDirectory<<DirDbStore as Store>::CID> {
    async fn xfer_into_stdout(self, _: &mut Dagio<DirDbStore>) -> anyhow::Result<()> {
        anyhow::bail!("cannot xfer host dir into stdout");
    }

    async fn xfer_into_host(
        self,
        dagio: &mut Dagio<DirDbStore>,
        path: &Path,
    ) -> anyhow::Result<()> {
        fs::create_dir(path).await?;

        for (name, link) in self {
            let dst = path.join(name);
            link.xfer_into(dagio, &Destination::Host(dst)).await?;
        }

        Ok(())
    }

    async fn xfer_into_store(
        self,
        dagio: &mut Dagio<DirDbStore>,
        dest: Option<&StoreDestination<CidMeta<<DirDbStore as Store>::CID>>>,
    ) -> anyhow::Result<Link<CidMeta<<DirDbStore as Store>::CID>>> {
        dagio.commit_into(self, dest).await
    }
}
