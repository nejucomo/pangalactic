use std::{
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
    fsutil,
    NSource::{self, Branch, Leaf},
};

pub trait IntoSource<L, B> {
    fn into_source<S>(
        self,
        store: &LinkDirectoryLayer<S>,
    ) -> impl Future<Output = Result<NSource<L, B>>> + Send
    where
        S: Store;
}

impl<'a> IntoSource<File, ReadDir> for &'a Path {
    fn into_source<S>(
        self,
        store: &LinkDirectoryLayer<S>,
    ) -> impl Future<Output = Result<NSource<File, ReadDir>>> + Send
    where
        S: Store,
    {
        self.to_path_buf().into_source(store)
    }
}

impl IntoSource<File, ReadDir> for PathBuf {
    async fn into_source<S>(self, _: &LinkDirectoryLayer<S>) -> Result<NSource<File, ReadDir>>
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

impl IntoSource<File, ReadDir> for ReadDir {
    fn into_source<S>(
        self,
        _: &LinkDirectoryLayer<S>,
    ) -> impl Future<Output = Result<NSource<File, ReadDir>>> + Send
    where
        S: Store,
    {
        ready(Ok(Branch(self)))
    }
}

impl IntoSource<File, ReadDir> for File {
    fn into_source<S>(
        self,
        _: &LinkDirectoryLayer<S>,
    ) -> impl Future<Output = Result<NSource<File, ReadDir>>> + Send
    where
        S: Store,
    {
        ready(Ok(Leaf(self)))
    }
}

impl<R, B> IntoSource<R, B> for Readable<R>
where
    R: Send + AsyncRead,
    B: Send,
{
    fn into_source<S>(
        self,
        _: &LinkDirectoryLayer<S>,
    ) -> impl Future<Output = Result<NSource<R, B>>> + Send
    where
        S: Store,
    {
        ready(Ok(Leaf(self.0)))
    }
}
