use std::{
    future::Future,
    path::{Path, PathBuf},
};

use anyhow::Result;
use pangalactic_iowrappers::{Readable, Writable};
use pangalactic_name::Name;
use tokio::{
    fs::{File, ReadDir},
    io::{AsyncRead, AsyncWrite},
};

use crate::{fsutil, BranchIter, BranchSource, IntoSource, LeafOrBranchSource, LeafSource, Sink};

impl<'a> IntoSource for &'a Path {
    type Source = LeafOrBranchSource<File, ReadDir>;

    fn into_source(self) -> impl Future<Output = Result<Self::Source>> + Send {
        self.to_owned().into_source()
    }
}

impl IntoSource for PathBuf {
    type Source = LeafOrBranchSource<File, ReadDir>;

    async fn into_source(self) -> Result<Self::Source> {
        if self.is_file() {
            let f = File::open(self).await?;
            let leaf = f.into_source().await?;
            Ok(leaf.into())
        } else {
            let rd = tokio::fs::read_dir(self).await?;
            let branch = rd.into_source().await?;
            Ok(branch.into())
        }
    }
}

impl<B> Sink<BranchSource<B>> for PathBuf
where
    B: BranchIter,
    PathBuf: Sink<<B::IntoSource as IntoSource>::Source>,
{
    type CID = PathBuf;

    async fn sink(self, mut source: BranchSource<B>) -> Result<Self::CID> {
        fsutil::create_dir(&self).await?;

        while let Some((name, subintosrc)) = source.0.next_branch_entry().await? {
            let subsrc = subintosrc.into_source().await?;
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

impl IntoSource for ReadDir {
    type Source = BranchSource<ReadDir>;

    fn into_source(self) -> impl Future<Output = Result<Self::Source>> + Send {
        std::future::ready(Ok(BranchSource(self)))
    }
}

impl BranchIter for ReadDir {
    type IntoSource = PathBuf;

    async fn next_branch_entry(&mut self) -> Result<Option<(Name, PathBuf)>> {
        if let Some(entry) = self.next_entry().await? {
            let name = Name::from_os_str(entry.file_name())?;
            Ok(Some((name, entry.path())))
        } else {
            Ok(None)
        }
    }
}

impl IntoSource for File {
    type Source = LeafSource<File>;

    fn into_source(self) -> impl Future<Output = Result<Self::Source>> + Send {
        LeafSource(self).into_source()
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

impl<R> IntoSource for Readable<R>
where
    R: AsyncRead + Send,
{
    type Source = LeafSource<R>;

    fn into_source(self) -> impl Future<Output = Result<Self::Source>> + Send {
        LeafSource(self.0).into_source()
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
