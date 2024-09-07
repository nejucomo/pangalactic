use std::path::PathBuf;

use anyhow::Result;
use pangalactic_dag_transfer::{BranchIter, IntoSource};
use pangalactic_layer_dir::LinkDirectoryLayer;
use pangalactic_link::Link;
use pangalactic_linkpath::LinkPath;
use pangalactic_name::Name;
use pangalactic_store::{Commit, Store};

use crate::hos::Hos;

use super::SourceEndpoint;

#[derive(Debug)]
pub struct SourceEndpointBranch<S>(Inner<S>)
where
    S: Store;

type Inner<S> =
    Hos<<PathBuf as IntoSource<S>>::Branch, <LinkPath<<S as Store>::CID> as IntoSource<S>>::Branch>;

impl<S> Commit<LinkDirectoryLayer<S>> for SourceEndpointBranch<S>
where
    S: Store,
{
    async fn commit_into_store(self, store: &mut LinkDirectoryLayer<S>) -> Result<Link<S::CID>> {
        self.0.commit_into_store(store).await
    }
}

impl<S> BranchIter<S> for SourceEndpointBranch<S>
where
    S: Store,
{
    type IntoSource = SourceEndpoint<S::CID>;

    async fn next_branch_entry(&mut self) -> Result<Option<(Name, Self::IntoSource)>> {
        use pangalactic_dag_transfer::BranchIterOutput;

        <Inner<S> as BranchIter<S>>::next_branch_entry(&mut self.0)
            .await
            .map_branch_item(|hos| hos.map_store(LinkPath::from))
            .map_branch_item(SourceEndpoint::from)
    }
}

impl<S> From<Inner<S>> for SourceEndpointBranch<S>
where
    S: Store,
{
    fn from(inner: Inner<S>) -> Self {
        Self(inner)
    }
}
