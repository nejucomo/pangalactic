use anyhow::Result;
use pangalactic_dag_transfer::{IntoSource, Source};
use pangalactic_endpoint::OriginEndpoint;
use pangalactic_layer_dir::LinkDirectoryLayer;
use pangalactic_store::Store;
use serde::Serialize;

use crate::{FilteredOriginValue, GlobSet};

#[derive(Debug)]
pub struct FilteredOrigin<'g, C>(FilteredOriginValue<'g, C, ()>)
where
    C: Serialize;

impl<'g, C> FilteredOrigin<'g, C>
where
    C: Serialize,
{
    pub(crate) fn new(globset: &'g GlobSet, origin: OriginEndpoint<C>) -> Self {
        FilteredOrigin(FilteredOriginValue::new(globset, origin, ()))
    }
}

impl<'g, S> IntoSource<S> for FilteredOrigin<'g, S::CID>
where
    S: Store,
{
    type Leaf = <OriginEndpoint<S::CID> as IntoSource<S>>::Leaf;
    type Branch =
        FilteredOriginValue<'g, S::CID, <OriginEndpoint<S::CID> as IntoSource<S>>::Branch>;

    async fn into_source(
        self,
        store: &LinkDirectoryLayer<S>,
    ) -> Result<Source<Self::Leaf, Self::Branch>> {
        self.0.with_origin_as_value().into_source(store).await
    }
}
