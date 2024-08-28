use std::future::Future;

use anyhow::Result;
use pangalactic_asynctryiter::AsyncTryIterator;
use pangalactic_name::Name;
use tokio::fs::{File, ReadDir};

use crate::{IntoSource, LeafOrBranchSource};

#[derive(Debug, derive_more::From)]
pub struct ReadDirAdapter(ReadDir);

impl AsyncTryIterator for ReadDirAdapter {
    type Item = (Name, LeafOrBranchSource<File, ReadDirAdapter>);

    fn try_next_async(&mut self) -> impl Future<Output = Result<Option<Self::Item>>> + Send {
        async {
            if let Some((name, path)) = self.0.try_next_async().await? {
                let src = Box::pin(path.into_source()).await?;
                Ok(Some((name, src)))
            } else {
                Ok(None)
            }
        }
    }
}
