use std::{
    fmt::Debug,
    future::{ready, Future},
    path::{Path, PathBuf},
};

use anyhow::Result;
use pangalactic_layer_dir::{DirNodeReader, DirectoryIntoIter, LinkDirectoryLayer};
use pangalactic_link::Link;
use pangalactic_store::Store;
use tokio::{
    fs::{File, ReadDir},
    io::{AsyncRead, Stdin},
};

use crate::{
    fsutil, BranchIter,
    Source::{self, Branch, Leaf},
};

pub trait IntoSource<S>: Send
where
    S: Store,
{
    type Leaf: Send + Debug + AsyncRead;
    type Branch: Send + Debug + BranchIter<S>;

    fn into_source(
        self,
        store: &LinkDirectoryLayer<S>,
    ) -> impl Future<Output = Result<Source<Self::Leaf, Self::Branch>>> + Send;
}

impl<S> IntoSource<S> for &Path
where
    S: Store,
{
    type Leaf = File;
    type Branch = ReadDir;

    fn into_source(
        self,
        store: &LinkDirectoryLayer<S>,
    ) -> impl Future<Output = Result<Source<File, ReadDir>>> + Send {
        self.to_path_buf().into_source(store)
    }
}

impl<S> IntoSource<S> for PathBuf
where
    S: Store,
{
    type Leaf = File;
    type Branch = ReadDir;

    async fn into_source(self, _: &LinkDirectoryLayer<S>) -> Result<Source<File, ReadDir>> {
        if self.is_file() {
            let f = fsutil::open_readable_file(self).await?;
            Ok(Leaf(f))
        } else {
            let rd = tokio::fs::read_dir(self).await?;
            Ok(Branch(rd))
        }
    }
}

impl<S> IntoSource<S> for ReadDir
where
    S: Store,
{
    type Leaf = File;
    type Branch = ReadDir;

    fn into_source(
        self,
        _: &LinkDirectoryLayer<S>,
    ) -> impl Future<Output = Result<Source<File, ReadDir>>> + Send {
        ready(Ok(Branch(self)))
    }
}

impl<S> IntoSource<S> for File
where
    S: Store,
{
    type Leaf = File;
    type Branch = ReadDir;

    fn into_source(
        self,
        _: &LinkDirectoryLayer<S>,
    ) -> impl Future<Output = Result<Source<File, ReadDir>>> + Send {
        ready(Ok(Leaf(self)))
    }
}

impl<S> IntoSource<S> for Stdin
where
    S: Store,
{
    type Leaf = Stdin;
    type Branch = ReadDir; // Dummy value.

    fn into_source(
        self,
        _: &LinkDirectoryLayer<S>,
    ) -> impl Future<Output = Result<Source<Self::Leaf, Self::Branch>>> + Send {
        ready(Ok(Leaf(self)))
    }
}

impl<S> IntoSource<S> for Link<S::CID>
where
    S: Store,
{
    type Leaf = <LinkDirectoryLayer<S> as Store>::Reader;
    type Branch = DirectoryIntoIter<Link<S::CID>>;

    async fn into_source(
        self,
        store: &LinkDirectoryLayer<S>,
    ) -> Result<Source<Self::Leaf, Self::Branch>> {
        use DirNodeReader::*;

        let dnr: DirNodeReader<_> = store.load(&self).await?;
        Ok(match dnr {
            File(r) => Leaf(r),
            Dir(d) => Branch(d.into_iter()),
        })
    }
}
