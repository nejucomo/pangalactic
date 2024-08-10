use std::{path::PathBuf, pin::pin};

use anyhow::Result;
use pangalactic_bindref::Bindable;
use pangalactic_hostdir::{DirNodeReader, HostDirectory};
use pangalactic_iowrappers::{Readable, Writable};
use pangalactic_path::{AnyDestination, AnySource, PathLayer, StoreDestination, StorePath};
use pangalactic_store::{Commit, Store};
use tokio::{
    fs::{File, ReadDir},
    io::{AsyncRead, AsyncWrite, Stdin, Stdout},
};

use crate::Destination;

pub async fn transfer<S>(
    store: &mut PathLayer<S>,
    source: AnySource<S::CID>,
    destination: AnyDestination<S::CID>,
) -> Result<Option<StorePath<S::CID>>>
where
    S: Store,
{
    source.transfer_into(store, destination).await
}

pub trait TransferInto<S, D>
where
    S: Store,
    D: Destination,
{
    async fn transfer_into(self, store: &mut PathLayer<S>, destination: D) -> Result<D::CID>;
}

impl<S, D> TransferInto<S, D> for AnySource<S::CID>
where
    S: Store,
    D: Destination,
    Readable<Stdin>: TransferInto<S, D>,
    PathBuf: TransferInto<S, D>,
    StorePath<S::CID>: TransferInto<S, D>,
{
    async fn transfer_into(self, store: &mut PathLayer<S>, destination: D) -> Result<D::CID> {
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

impl<S, D> TransferInto<S, D> for StorePath<S::CID>
where
    S: Store,
    D: Destination,
    Readable<S::Reader>: TransferInto<S, D>,
    HostDirectory<S::CID>: TransferInto<S, D>,
{
    async fn transfer_into(self, store: &mut PathLayer<S>, destination: D) -> Result<D::CID> {
        let dnr: DirNodeReader<_> = store.load(&self).await?;
        match dnr {
            DirNodeReader::File(r) => r.transfer_into(store, destination).await,
            DirNodeReader::Dir(hd) => Box::pin(hd.transfer_into(store, destination)).await,
        }
    }
}

impl<S> TransferInto<S, AnyDestination<S::CID>> for HostDirectory<S::CID>
where
    S: Store,
{
    async fn transfer_into(
        self,
        store: &mut PathLayer<S>,
        destination: AnyDestination<S::CID>,
    ) -> Result<Option<StorePath<S::CID>>> {
        transfer_to_any_destination(self, store, destination).await
    }
}

impl<S> TransferInto<S, StoreDestination<S::CID>> for HostDirectory<S::CID>
where
    S: Store,
{
    async fn transfer_into(
        self,
        store: &mut PathLayer<S>,
        destination: StoreDestination<S::CID>,
    ) -> Result<StorePath<S::CID>> {
        store.commit(destination.bind_ref(self)).await
    }
}

impl<S> TransferInto<S, PathBuf> for HostDirectory<S::CID>
where
    S: Store,
{
    async fn transfer_into(self, store: &mut PathLayer<S>, destination: PathBuf) -> Result<()> {
        tokio::fs::create_dir(&destination).await?;

        for (name, link) in self {
            StorePath::from(link)
                .transfer_into(store, destination.join(name))
                .await?;
        }

        Ok(())
    }
}

impl<S, W> TransferInto<S, Writable<W>> for HostDirectory<S::CID>
where
    S: Store,
{
    async fn transfer_into(self, _: &mut PathLayer<S>, _: Writable<W>) -> Result<()> {
        anyhow::bail!("cannot transfer store directory into stream");
    }
}

impl<S> TransferInto<S, AnyDestination<S::CID>> for PathBuf
where
    S: Store,
{
    async fn transfer_into(
        self,
        store: &mut PathLayer<S>,
        destination: AnyDestination<S::CID>,
    ) -> Result<Option<StorePath<S::CID>>> {
        transfer_to_any_destination(self, store, destination).await
    }
}

impl<S> TransferInto<S, StoreDestination<S::CID>> for PathBuf
where
    S: Store,
{
    async fn transfer_into(
        self,
        store: &mut PathLayer<S>,
        destination: StoreDestination<S::CID>,
    ) -> Result<StorePath<S::CID>> {
        store.commit(destination.bind_ref(self)).await
    }
}

impl<S, W> TransferInto<S, Writable<W>> for PathBuf
where
    S: Store,
    W: AsyncWrite,
{
    async fn transfer_into(self, store: &mut PathLayer<S>, destination: Writable<W>) -> Result<()> {
        let f = File::open(self).await?;
        Readable(f).transfer_into(store, destination).await
    }
}

impl<S> TransferInto<S, PathBuf> for PathBuf
where
    S: Store,
{
    async fn transfer_into(self, store: &mut PathLayer<S>, destination: PathBuf) -> Result<()> {
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
    async fn transfer_into(mut self, store: &mut PathLayer<S>, destination: PathBuf) -> Result<()> {
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
    R: AsyncRead + Send,
{
    async fn transfer_into(
        self,
        store: &mut PathLayer<S>,
        destination: AnyDestination<S::CID>,
    ) -> Result<Option<StorePath<S::CID>>> {
        transfer_to_any_destination(self, store, destination).await
    }
}

impl<S, R> TransferInto<S, StoreDestination<S::CID>> for Readable<R>
where
    S: Store,
    R: AsyncRead + Send,
{
    async fn transfer_into(
        self,
        store: &mut PathLayer<S>,
        destination: StoreDestination<S::CID>,
    ) -> Result<StorePath<S::CID>> {
        store.commit(destination.bind_ref(self)).await
    }
}

impl<S, R> TransferInto<S, PathBuf> for Readable<R>
where
    S: Store,
    R: AsyncRead + Send,
{
    async fn transfer_into(self, store: &mut PathLayer<S>, destination: PathBuf) -> Result<()> {
        let f = File::create(destination).await?;
        self.transfer_into(store, Writable(f)).await
    }
}

impl<S, W, R> TransferInto<S, Writable<W>> for Readable<R>
where
    S: Store,
    W: AsyncWrite,
    R: AsyncRead + Send,
{
    async fn transfer_into(self, _: &mut PathLayer<S>, destination: Writable<W>) -> Result<()> {
        tokio::io::copy(&mut pin!(self), &mut pin!(destination)).await?;
        Ok(())
    }
}

async fn transfer_to_any_destination<T, S>(
    source: T,
    store: &mut PathLayer<S>,
    destination: AnyDestination<S::CID>,
) -> Result<Option<StorePath<S::CID>>>
where
    S: Store,
    T: TransferInto<S, Writable<Stdout>> + TransferInto<S, PathBuf> + Commit<PathLayer<S>> + Send,
{
    match destination {
        AnyDestination::Stdout => source
            .transfer_into(store, Writable(tokio::io::stdout()))
            .await
            .map(|()| None),
        AnyDestination::Host(p) => source.transfer_into(store, p).await.map(|()| None),
        AnyDestination::Store(None) => store.commit(source).await.map(Some),
        AnyDestination::Store(Some(dest)) => store.commit(dest.bind_ref(source)).await.map(Some),
    }
}
