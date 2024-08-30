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

pub trait IntoSource: Send {
    type Leaf: Send + Debug + AsyncRead;
    type Branch: Send + Debug + BranchIter;

    fn into_source<S>(
        self,
        store: &LinkDirectoryLayer<S>,
    ) -> impl Future<Output = Result<Source<Self::Leaf, Self::Branch>>> + Send
    where
        S: Store;
}

impl IntoSource for () {
    type Leaf = File; // Dummy value
    type Branch = ();

    async fn into_source<S>(
        self,
        _: &LinkDirectoryLayer<S>,
    ) -> Result<Source<Self::Leaf, Self::Branch>>
    where
        S: Store,
    {
        unimplemented!("a () IntoSource should never be instantiated")
    }
}

impl<'a> IntoSource for &'a Path {
    type Leaf = File;
    type Branch = ReadDir;

    fn into_source<S>(
        self,
        store: &LinkDirectoryLayer<S>,
    ) -> impl Future<Output = Result<Source<File, ReadDir>>> + Send
    where
        S: Store,
    {
        self.to_path_buf().into_source(store)
    }
}

impl IntoSource for PathBuf {
    type Leaf = File;
    type Branch = ReadDir;

    async fn into_source<S>(self, _: &LinkDirectoryLayer<S>) -> Result<Source<File, ReadDir>>
    where
        S: Store,
    {
        if self.is_file() {
            let f = fsutil::open_readable_file(self).await?;
            Ok(Leaf(f))
        } else {
            let rd = tokio::fs::read_dir(self).await?;
            Ok(Branch(rd))
        }
    }
}

impl IntoSource for ReadDir {
    type Leaf = File;
    type Branch = ReadDir;

    fn into_source<S>(
        self,
        _: &LinkDirectoryLayer<S>,
    ) -> impl Future<Output = Result<Source<File, ReadDir>>> + Send
    where
        S: Store,
    {
        ready(Ok(Branch(self)))
    }
}

impl IntoSource for File {
    type Leaf = File;
    type Branch = ReadDir;

    fn into_source<S>(
        self,
        _: &LinkDirectoryLayer<S>,
    ) -> impl Future<Output = Result<Source<File, ReadDir>>> + Send
    where
        S: Store,
    {
        ready(Ok(Leaf(self)))
    }
}

impl<R> IntoSource for Readable<R>
where
    R: Send + Debug + AsyncRead,
{
    type Leaf = R;
    type Branch = ();

    fn into_source<S>(
        self,
        _: &LinkDirectoryLayer<S>,
    ) -> impl Future<Output = Result<Source<R, ()>>> + Send
    where
        S: Store,
    {
        ready(Ok(Leaf(self.0)))
    }
}
