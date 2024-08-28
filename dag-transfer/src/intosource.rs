use std::{
    future::Future,
    path::{Path, PathBuf},
};

use anyhow::Result;
use pangalactic_iowrappers::Readable;
use tokio::{
    fs::{File, ReadDir},
    io::AsyncRead,
};

use crate::{BranchSource, LeafOrBranchSource, LeafSource, ReadDirAdapter, Source};

pub trait IntoSource {
    type Source: Source;

    fn into_source(self) -> impl Future<Output = Result<Self::Source>> + Send;
}

impl IntoSource for PathBuf {
    type Source = LeafOrBranchSource<File, ReadDirAdapter>;

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

impl<'a> IntoSource for &'a Path {
    type Source = LeafOrBranchSource<File, ReadDirAdapter>;

    fn into_source(self) -> impl Future<Output = Result<Self::Source>> + Send {
        self.to_owned().into_source()
    }
}

impl IntoSource for ReadDir {
    type Source = BranchSource<File, ReadDirAdapter>;

    fn into_source(self) -> impl Future<Output = Result<Self::Source>> + Send {
        std::future::ready(Ok(BranchSource(ReadDirAdapter::from(self))))
    }
}

impl IntoSource for File {
    type Source = LeafSource<File>;

    fn into_source(self) -> impl Future<Output = Result<Self::Source>> + Send {
        LeafSource(self).into_source()
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
