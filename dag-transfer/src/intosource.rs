use std::{
    fmt::Debug,
    future::{ready, Future},
    path::{Path, PathBuf},
};

use anyhow::Result;
use pangalactic_iowrappers::Readable;
use pangalactic_layer_dir::LinkDirectoryLayer;
use pangalactic_store::Store;
use tokio::{
    fs::{File, ReadDir},
    io::AsyncRead,
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

impl<S> IntoSource<S> for ()
where
    S: Store,
{
    type Leaf = File; // Dummy value
    type Branch = ();

    async fn into_source(
        self,
        _: &LinkDirectoryLayer<S>,
    ) -> Result<Source<Self::Leaf, Self::Branch>> {
        unimplemented!("a () IntoSource should never be instantiated")
    }
}

impl<'a, S> IntoSource<S> for &'a Path
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

impl<S, R> IntoSource<S> for Readable<R>
where
    S: Store,
    R: Send + Debug + AsyncRead,
{
    type Leaf = R;
    type Branch = ();

    fn into_source(
        self,
        _: &LinkDirectoryLayer<S>,
    ) -> impl Future<Output = Result<Source<R, ()>>> + Send {
        ready(Ok(Leaf(self.0)))
    }
}
