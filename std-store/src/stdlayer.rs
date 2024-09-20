use anyhow::Result;
use pangalactic_dag_transfer::{Destination, IntoSource};
use pangalactic_hash::Hash;
use pangalactic_host::HostLayerExt;
use pangalactic_layer_cidmeta::CidMetaLayer;
use pangalactic_layer_dir::LinkDirectoryLayer;
use pangalactic_store::{Commit, Load, Store};
use pangalactic_store_dirdb::DirDbStore;
use pangalactic_store_mem::MemStore;

use crate::StdLink;

pub type StdStore = StdLayer<DirDbStore>;
pub type StdMemStore = StdLayer<MemStore>;

#[derive(Debug, Default)]
pub struct StdLayer<S>(StdLayerInner<S>)
where
    S: Store<CID = Hash>;

pub type StdLayerInner<S> = LinkDirectoryLayer<CidMetaLayer<S>>;

impl<S> StdLayer<S>
where
    S: Store<CID = Hash>,
{
    pub async fn commit<T>(&mut self, object: T) -> Result<StdLink>
    where
        T: Commit<StdLayerInner<S>> + Send,
    {
        self.0.commit(object).await
    }

    pub async fn load<T>(&self, link: &StdLink) -> Result<T>
    where
        T: Load<StdLayerInner<S>>,
    {
        self.0.load(link).await
    }

    pub async fn transfer<I, D>(&mut self, source: I, destination: D) -> Result<D::CID>
    where
        I: IntoSource<CidMetaLayer<S>>,
        D: Destination<CidMetaLayer<S>>,
    {
        let s = source.into_source(&self.0).await?;
        destination.sink(&mut self.0, s).await
    }

    pub async fn derive(self, plan: &StdLink) -> Result<(Self, StdLink)> {
        let (inner, attestation) = self.0.derive(plan).await?;
        let newself = StdLayer(inner);
        Ok((newself, attestation))
    }
}
