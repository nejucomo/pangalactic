use std::path::PathBuf;

use anyhow::Result;
use pangalactic_dag_transfer::{BranchIter, IntoSource};
use pangalactic_layer_dir::LinkDirectoryLayer;
use pangalactic_link::Link;
use pangalactic_linkpath::LinkPath;
use pangalactic_name::Name;
use pangalactic_store::{Commit, Store};

use crate::SourceEndpoint;

#[derive(Debug)]
pub enum SourceEndpointBranch<S>
where
    S: Store,
{
    Host(<PathBuf as IntoSource<S>>::Branch),
    Store(<LinkPath<S::CID> as IntoSource<S>>::Branch),
}

impl<S> Commit<LinkDirectoryLayer<S>> for SourceEndpointBranch<S>
where
    S: Store,
{
    async fn commit_into_store(self, store: &mut LinkDirectoryLayer<S>) -> Result<Link<S::CID>> {
        use self::SourceEndpointBranch::*;

        match self {
            Host(x) => x.commit_into_store(store).await,
            Store(x) => x.commit_into_store(store).await,
        }
    }
}

impl<S> BranchIter<S> for SourceEndpointBranch<S>
where
    S: Store,
{
    type IntoSource = SourceEndpoint<S::CID>;

    async fn next_branch_entry(&mut self) -> Result<Option<(Name, Self::IntoSource)>> {
        use self::SourceEndpointBranch::*;

        match self {
            Host(x) => delegate_next_branch_entry::<_, _, S>(x, SourceEndpoint::Host).await,
            Store(x) => {
                delegate_next_branch_entry::<_, _, S>(x, |link| {
                    SourceEndpoint::Store(LinkPath::from(link))
                })
                .await
            }
        }
    }
}

async fn delegate_next_branch_entry<B, F, S>(
    inner: &mut B,
    f: F,
) -> Result<Option<(Name, SourceEndpoint<S::CID>)>>
where
    B: BranchIter<S>,
    F: FnOnce(B::IntoSource) -> SourceEndpoint<S::CID>,
    S: Store,
{
    if let Some((name, inneritem)) = inner.next_branch_entry().await? {
        Ok(Some((name, f(inneritem))))
    } else {
        Ok(None)
    }
}
