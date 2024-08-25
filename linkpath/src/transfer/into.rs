use std::{fmt::Debug, path::PathBuf, pin::pin};

use anyhow::Result;
use pangalactic_iowrappers::{Readable, Writable};
use pangalactic_layer_dir::{DirNodeReader, LinkDirectory, LinkDirectoryLayer};
use pangalactic_store::{Commit, Store};
use tokio::{
    fs::{File, ReadDir},
    io::{AsyncRead, AsyncWrite, Stdin, Stdout},
};

use crate::transfer::Destination;
use crate::{AnyDestination, AnySource, LinkDestination, LinkPath, PathLayerExt};

pub trait TransferInto<S, D>
where
    S: Store,
    D: Destination,
{
    async fn transfer_into(
        self,
        store: &mut LinkDirectoryLayer<S>,
        destination: D,
    ) -> Result<D::CID>;
}

impl<S, D> TransferInto<S, D> for AnySource<S::CID>
where
    S: Store,
    D: Destination,
    Readable<Stdin>: TransferInto<S, D>,
    PathBuf: TransferInto<S, D>,
    LinkPath<S::CID>: TransferInto<S, D>,
{
    async fn transfer_into(
        self,
        store: &mut LinkDirectoryLayer<S>,
        destination: D,
    ) -> Result<D::CID> {
        tracing::debug!(source = ?&self, ?destination, "transferring");
        match self {
            AnySource::Stdin => {
                Readable(tokio::io::stdin())
                    .transfer_into(store, destination)
                    .await
            }
            AnySource::Host(p) => p.transfer_into(store, destination).await,
            AnySource::Store(sp) => sp.transfer_into(store, destination).await,
        }
    }
}

impl<S, D> TransferInto<S, D> for LinkPath<S::CID>
where
    S: Store,
    D: Destination,
    Readable<S::Reader>: TransferInto<S, D>,
    LinkDirectory<S::CID>: TransferInto<S, D>,
{
    async fn transfer_into(
        self,
        store: &mut LinkDirectoryLayer<S>,
        destination: D,
    ) -> Result<D::CID> {
        let dnr: DirNodeReader<_> = store.load_path(&self).await?;
        match dnr {
            DirNodeReader::File(r) => r.transfer_into(store, destination).await,
            DirNodeReader::Dir(hd) => Box::pin(hd.transfer_into(store, destination)).await,
        }
    }
}

impl<S> TransferInto<S, AnyDestination<S::CID>> for LinkDirectory<S::CID>
where
    S: Store,
{
    async fn transfer_into(
        self,
        store: &mut LinkDirectoryLayer<S>,
        destination: AnyDestination<S::CID>,
    ) -> Result<Option<LinkPath<S::CID>>> {
        transfer_to_any_destination(self, store, destination).await
    }
}

impl<S> TransferInto<S, LinkDestination<S::CID>> for LinkDirectory<S::CID>
where
    S: Store,
{
    async fn transfer_into(
        self,
        store: &mut LinkDirectoryLayer<S>,
        destination: LinkDestination<S::CID>,
    ) -> Result<LinkPath<S::CID>> {
        store
            .commit_into_dest(self, destination)
            .await
            .map(LinkPath::from)
    }
}

impl<S> TransferInto<S, PathBuf> for LinkDirectory<S::CID>
where
    S: Store,
{
    async fn transfer_into(
        self,
        store: &mut LinkDirectoryLayer<S>,
        destination: PathBuf,
    ) -> Result<()> {
        tokio::fs::create_dir(&destination).await?;

        for (name, link) in self {
            LinkPath::from(link)
                .transfer_into(store, destination.join(name.as_str()))
                .await?;
        }

        Ok(())
    }
}

impl<S, W> TransferInto<S, Writable<W>> for LinkDirectory<S::CID>
where
    S: Store,
    W: Debug,
{
    async fn transfer_into(self, _: &mut LinkDirectoryLayer<S>, _: Writable<W>) -> Result<()> {
        anyhow::bail!("cannot transfer store directory into stream");
    }
}

impl<S> TransferInto<S, AnyDestination<S::CID>> for PathBuf
where
    S: Store,
{
    async fn transfer_into(
        self,
        store: &mut LinkDirectoryLayer<S>,
        destination: AnyDestination<S::CID>,
    ) -> Result<Option<LinkPath<S::CID>>> {
        transfer_to_any_destination(self, store, destination).await
    }
}

impl<S, W> TransferInto<S, Writable<W>> for PathBuf
where
    S: Store,
    W: AsyncWrite + Debug,
{
    async fn transfer_into(
        self,
        store: &mut LinkDirectoryLayer<S>,
        destination: Writable<W>,
    ) -> Result<()> {
        let f = File::open(self).await?;
        Readable(f).transfer_into(store, destination).await
    }
}

impl<S> TransferInto<S, PathBuf> for PathBuf
where
    S: Store,
{
    async fn transfer_into(
        self,
        store: &mut LinkDirectoryLayer<S>,
        destination: PathBuf,
    ) -> Result<()> {
        if self.is_file() {
            let f = File::open(self).await?;
            Readable(f).transfer_into(store, destination).await
        } else if self.is_dir() {
            let r = tokio::fs::read_dir(self).await?;
            Box::pin(r.transfer_into(store, destination)).await
        } else {
            anyhow::bail!("unknown fs node type: {:?}", self.display());
        }
    }
}

impl<S> TransferInto<S, PathBuf> for ReadDir
where
    S: Store,
{
    async fn transfer_into(
        mut self,
        store: &mut LinkDirectoryLayer<S>,
        destination: PathBuf,
    ) -> Result<()> {
        tokio::fs::create_dir(&destination).await?;

        while let Some(entry) = self.next_entry().await? {
            entry
                .path()
                .transfer_into(store, destination.join(entry.file_name()))
                .await?;
        }

        Ok(())
    }
}

impl<S, R> TransferInto<S, AnyDestination<S::CID>> for Readable<R>
where
    S: Store,
    R: AsyncRead + Debug + Send,
{
    async fn transfer_into(
        self,
        store: &mut LinkDirectoryLayer<S>,
        destination: AnyDestination<S::CID>,
    ) -> Result<Option<LinkPath<S::CID>>> {
        transfer_to_any_destination(self, store, destination).await
    }
}

impl<S, R> TransferInto<S, PathBuf> for Readable<R>
where
    S: Store,
    R: AsyncRead + Send,
{
    async fn transfer_into(
        self,
        store: &mut LinkDirectoryLayer<S>,
        destination: PathBuf,
    ) -> Result<()> {
        let f = File::create(destination).await?;
        self.transfer_into(store, Writable(f)).await
    }
}

impl<S, W, R> TransferInto<S, Writable<W>> for Readable<R>
where
    S: Store,
    W: AsyncWrite + Debug,
    R: AsyncRead + Send,
{
    async fn transfer_into(
        self,
        _: &mut LinkDirectoryLayer<S>,
        destination: Writable<W>,
    ) -> Result<()> {
        tokio::io::copy(&mut pin!(self), &mut pin!(destination)).await?;
        Ok(())
    }
}

async fn transfer_to_any_destination<T, S>(
    source: T,
    store: &mut LinkDirectoryLayer<S>,
    destination: AnyDestination<S::CID>,
) -> Result<Option<LinkPath<S::CID>>>
where
    S: Store,
    T: TransferInto<S, Writable<Stdout>>
        + TransferInto<S, PathBuf>
        + Commit<LinkDirectoryLayer<S>>
        + Send,
{
    match destination {
        AnyDestination::Stdout => source
            .transfer_into(store, Writable(tokio::io::stdout()))
            .await
            .map(|()| None),
        AnyDestination::Host(p) => source.transfer_into(store, p).await.map(|()| None),
        AnyDestination::Store(optdest) => {
            store.commit_into_optdest(source, optdest).await.map(Some)
        }
    }
}
