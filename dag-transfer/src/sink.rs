use std::{future::Future, path::PathBuf};

use anyhow::Result;
use pangalactic_asynctryiter::{AsyncTryIterator, IntoAsyncTryIterator};
use pangalactic_iowrappers::{Readable, Writable};
use pangalactic_name::Name;
use tokio::{
    fs::File,
    io::{AsyncRead, AsyncWrite},
};

use crate::{fsutil, BranchSource, IntoSource, LeafOrBranchSource, LeafSource, Source};

pub trait Sink<S>
where
    S: Source,
{
    type CID;

    fn sink(self, source: S) -> impl Future<Output = Result<Self::CID>>;
}

impl<R, I, T, S> Sink<LeafOrBranchSource<R, I, T>> for S
where
    R: AsyncRead + Send,
    I: IntoAsyncTryIterator<Item = (Name, T)> + Send,
    T: IntoSource,
    S: Sink<LeafSource<R>> + Sink<BranchSource<I, T>, CID = <S as Sink<LeafSource<R>>>::CID>,
{
    type CID = <S as Sink<LeafSource<R>>>::CID;

    async fn sink(self, source: LeafOrBranchSource<R, I, T>) -> Result<Self::CID> {
        use LeafOrBranchSource::*;

        match source {
            Leaf(l) => self.sink(LeafSource(l)).await,
            Branch(b) => self.sink(BranchSource(b)).await,
        }
    }
}

impl<I, T> Sink<BranchSource<I, T>> for PathBuf
where
    I: IntoAsyncTryIterator<Item = (Name, T)> + Send,
    T: IntoSource,
    PathBuf: Sink<T::Source>,
{
    type CID = PathBuf;

    async fn sink(self, source: BranchSource<I, T>) -> Result<Self::CID> {
        fsutil::create_dir(&self).await?;

        let mut ati = source.into_async_try_iter();
        while let Some((name, item)) = ati.ati_next().await? {
            let subsrc = item.into_source().await?;
            self.join(name).sink(subsrc).await?;
        }
        Ok(self)
    }
}

impl<R> Sink<LeafSource<R>> for PathBuf
where
    R: AsyncRead + Send,
{
    type CID = PathBuf;

    async fn sink(self, source: LeafSource<R>) -> Result<Self::CID> {
        let f = fsutil::create_file(&self).await?;
        f.sink(source).await?;
        Ok(self)
    }
}

impl<R> Sink<LeafSource<R>> for File
where
    R: AsyncRead + Send,
{
    type CID = ();

    async fn sink(self, source: LeafSource<R>) -> Result<Self::CID> {
        Writable(self).sink(source).await
    }
}

impl<R, W> Sink<LeafSource<R>> for Writable<W>
where
    R: AsyncRead + Send,
    W: AsyncWrite,
{
    type CID = ();

    async fn sink(self, source: LeafSource<R>) -> Result<Self::CID> {
        use std::pin::pin;

        let r = Readable(source.0);
        tokio::io::copy(&mut pin!(r), &mut pin!(self)).await?;
        Ok(())
    }
}
