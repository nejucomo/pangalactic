use std::{
    future::{ready, Future},
    path::PathBuf,
};

use anyhow::Result;
use pangalactic_layer_dir::DirectoryIntoIter;
use pangalactic_link::Link;
use pangalactic_name::Name;
use pangalactic_store::Store;
use tokio::fs::ReadDir;

use crate::IntoSource;

pub trait BranchIter<S>: Sized + Send
where
    S: Store,
{
    type IntoSource: IntoSource<S>;

    fn next_branch_entry(
        &mut self,
    ) -> impl Future<Output = Result<Option<(Name, Self::IntoSource)>>> + Send;
}

impl<S> BranchIter<S> for ()
where
    S: Store,
{
    type IntoSource = ();

    async fn next_branch_entry(&mut self) -> Result<Option<(Name, Self::IntoSource)>> {
        unimplemented!("a () BranchIter should never be instantiated")
    }
}

impl<S> BranchIter<S> for ReadDir
where
    S: Store,
{
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

impl<S> BranchIter<S> for DirectoryIntoIter<Link<S::CID>>
where
    S: Store,
{
    type IntoSource = Link<S::CID>;

    fn next_branch_entry(
        &mut self,
    ) -> impl Future<Output = Result<Option<(Name, Self::IntoSource)>>> + Send {
        ready(Ok(self.next()))
    }
}
