use std::{future::Future, path::PathBuf};

use anyhow::Result;
use pangalactic_name::Name;
use tokio::fs::ReadDir;

use crate::TryMapAsync;

pub trait AsyncTryIterator {
    type Item;

    fn try_next_async(&mut self) -> impl Future<Output = Result<Option<Self::Item>>> + Send;

    fn try_map_async<F, Fut, T>(self, f: F) -> TryMapAsync<Self, F, Fut, T>
    where
        Self: Send + Sized,
        Self::Item: Send,
        F: Send + Sync + Fn(Self::Item) -> Fut,
        Fut: Send + Future<Output = Result<T>>,
    {
        TryMapAsync::new(self, f)
    }
}

impl AsyncTryIterator for ReadDir {
    type Item = (Name, PathBuf);

    async fn try_next_async(&mut self) -> Result<Option<Self::Item>> {
        if let Some(entry) = self.next_entry().await? {
            let name = Name::from_os_str(entry.file_name())?;
            Ok(Some((name, entry.path())))
        } else {
            Ok(None)
        }
    }
}
