use anyhow::Result;
use pangalactic_hash::Hash;
use pangalactic_layer_cidmeta::{CidMeta, CidMetaLayer};
use pangalactic_path::{AnyDestination, AnySource, PathLayer, StorePath};
use pangalactic_store_dirdb::DirDbStore;
use pangalactic_transfer::Transferor;

#[derive(Debug, Default)]
pub struct StandardStore(Inner);

type Inner = PathLayer<CidMetaLayer<DirDbStore>>;

pub type StandardPath = StorePath<CidMeta<Hash>>;
pub type StandardAnySource = AnySource<CidMeta<Hash>>;
pub type StandardAnyDestination = AnyDestination<CidMeta<Hash>>;

impl StandardStore {
    pub async fn transfer(
        &mut self,
        source: StandardAnySource,
        destination: StandardAnyDestination,
    ) -> Result<Option<StandardPath>> {
        self.0.transfer(source, destination).await
    }
}
