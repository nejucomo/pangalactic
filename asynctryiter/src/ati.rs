use std::{future::Future, path::PathBuf};

use anyhow::Result;
use pangalactic_name::Name;
use tokio::fs::ReadDir;

pub trait AsyncTryIterator {
    type Item;

    fn next(&mut self) -> impl Future<Output = Result<Option<Self::Item>>> + Send;
}

impl AsyncTryIterator for ReadDir {
    type Item = (Name, PathBuf);

    async fn next(&mut self) -> Result<Option<Self::Item>> {
        if let Some(entry) = self.next_entry().await? {
            let name = Name::from_os_str(entry.file_name())?;
            Ok(Some((name, entry.path())))
        } else {
            Ok(None)
        }
    }
}
