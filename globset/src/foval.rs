use anyhow::Result;
use pangalactic_dag_transfer::{BranchIter, IntoSource, Source};
use pangalactic_endpoint::OriginEndpoint;
use pangalactic_layer_dir::LinkDirectoryLayer;
use pangalactic_name::Name;
use pangalactic_store::{Commit, Store};
use serde::Serialize;

use crate::GlobSet;

#[derive(Debug)]
pub struct FilteredOriginValue<'g, C, T>
where
    C: Serialize,
{
    globset: &'g GlobSet,
    origin: OriginEndpoint<C>,
    value: T,
}

impl<'g, C, T> FilteredOriginValue<'g, C, T>
where
    C: Serialize,
{
    pub fn new(globset: &'g GlobSet, origin: OriginEndpoint<C>, value: T) -> Self {
        Self {
            globset,
            origin,
            value,
        }
    }
}

impl<'g, C> FilteredOriginValue<'g, C, ()>
where
    C: Clone + Serialize,
{
    pub(crate) fn with_origin_as_value(self) -> FilteredOriginValue<'g, C, OriginEndpoint<C>> {
        let FilteredOriginValue {
            globset, origin, ..
        } = self;
        FilteredOriginValue::new(globset, origin.clone(), origin)
    }
}

impl<'g, S, I> IntoSource<S> for FilteredOriginValue<'g, S::CID, I>
where
    S: Store,
    I: IntoSource<S>,
{
    type Leaf = I::Leaf;
    type Branch = FilteredOriginValue<'g, S::CID, I::Branch>;

    async fn into_source(
        self,
        store: &LinkDirectoryLayer<S>,
    ) -> Result<Source<Self::Leaf, Self::Branch>> {
        use Source::*;

        let FilteredOriginValue {
            globset,
            origin,
            value,
        } = self;

        if globset.matches(&origin) {
            anyhow::bail!("{globset:?} filters out {origin:?}");
        }

        match value.into_source(store).await? {
            Leaf(l) => Ok(Leaf(l)),
            Branch(b) => Ok(Branch(FilteredOriginValue::new(globset, origin, b))),
        }
    }
}

impl<'g, S, B> BranchIter<S> for FilteredOriginValue<'g, S::CID, B>
where
    S: Store,
    B: BranchIter<S>,
{
    type IntoSource = FilteredOriginValue<'g, S::CID, B::IntoSource>;

    async fn next_branch_entry(&mut self) -> Result<Option<(Name, Self::IntoSource)>> {
        while let Some((name, intosrc)) = self.value.next_branch_entry().await? {
            let nameref = &name;
            let suborigin = self
                .origin
                .as_ref()
                .map_io(|io| *io)
                .map_host(|p| p.join(nameref))
                .map_store(|p| p.join(nameref));

            if self.globset.matches(&suborigin) {
                tracing::debug!(?suborigin, "filtered out");
            } else {
                let wrapper = FilteredOriginValue::new(self.globset, suborigin, intosrc);
                let item = (name, wrapper);
                return Ok(Some(item));
            }
        }
        Ok(None)
    }
}

impl<'g, S, B> Commit<LinkDirectoryLayer<S>> for FilteredOriginValue<'g, S::CID, B>
where
    S: Store,
    B: BranchIter<S>,
{
    async fn commit_into_store(
        self,
        store: &mut LinkDirectoryLayer<S>,
    ) -> Result<<LinkDirectoryLayer<S> as Store>::CID> {
        store.commit(self.value).await
    }
}
