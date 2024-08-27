use std::{async_iter::AsyncIterator, future::Future, path::PathBuf, task::Poll};

use anyhow::Result;
use pangalactic_name::Name;
use pin_project::pin_project;
use tokio::fs::{DirEntry, ReadDir};

use crate::{BranchSource, IntoSource};

#[pin_project]
#[derive(Debug)]
pub struct ReadDirAsyncIteratorAdapter(#[pin] pub(crate) ReadDir);

impl IntoSource for ReadDirAsyncIteratorAdapter {
    type Source = BranchSource<Self, PathBuf>;

    fn into_source(self) -> impl Future<Output = Result<Self::Source>> + Send {
        std::future::ready(Ok(BranchSource(self)))
    }
}

impl AsyncIterator for ReadDirAsyncIteratorAdapter {
    type Item = Result<(Name, PathBuf)>;

    fn poll_next(
        self: std::pin::Pin<&mut Self>,
        cx: &mut std::task::Context<'_>,
    ) -> Poll<Option<Self::Item>> {
        self.project()
            .0
            .poll_next_entry(cx)
            .map(|resopt| resopt.transpose().map(map_resentry))
    }
}

fn map_resentry(resopt: std::io::Result<DirEntry>) -> Result<(Name, PathBuf)> {
    use anyhow_std::OsStrAnyhow;

    let entry = resopt?;
    let name = entry.file_name().to_str_anyhow()?.parse()?;
    Ok((name, entry.path()))
}
