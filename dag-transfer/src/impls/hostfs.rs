use std::path::PathBuf;

use anyhow::Result;
use pangalactic_asynctryiter::{AsyncTryIterator, IntoAsyncTryIterator};
use pangalactic_iowrappers::{Readable, Writable};
use pangalactic_name::Name;
use tokio::{
    fs::File,
    io::{AsyncRead, AsyncWrite},
};

use crate::{fsutil, BranchSource, LeafOrBranchSource, LeafSource, Sink};

impl<R, I> Sink<BranchSource<R, I>> for PathBuf
where
    R: AsyncRead + Send,
    I: IntoAsyncTryIterator<Item = (Name, LeafOrBranchSource<R, I>)> + Send,
    PathBuf: Sink<LeafOrBranchSource<R, I>>,
{
    type CID = PathBuf;

    async fn sink(self, source: BranchSource<R, I>) -> Result<Self::CID> {
        fsutil::create_dir(&self).await?;

        let mut ati = source.into_async_try_iter();
        while let Some((name, subsrc)) = ati.try_next_async().await? {
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
