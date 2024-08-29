use std::{future::Future, path::PathBuf};

use anyhow::Result;
use pangalactic_name::Name;
use tokio::fs::{File, ReadDir};

use crate::IntoSource;

pub trait BranchIter<L, B>: Sized + Send {
    type IntoSource: IntoSource<L, B>;

    fn next_branch_entry(
        &mut self,
    ) -> impl Future<Output = Result<Option<(Name, Self::IntoSource)>>> + Send;
}

impl BranchIter<File, ReadDir> for ReadDir {
    type IntoSource = PathBuf;

    async fn next_branch_entry(&mut self) -> Result<Option<(Name, Self::IntoSource)>> {
        if let Some(entry) = self.next_entry().await? {
            let name = Name::from_os_str(entry.file_name())?;
            let item = (name, entry.path());
            Ok(Some(item))
        } else {
            Ok(None)
        }
    }
}
