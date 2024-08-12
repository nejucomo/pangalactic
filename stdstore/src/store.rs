use anyhow::Result;
use pangalactic_hash::Hash;
use pangalactic_layer_cidmeta::{CidMeta, CidMetaLayer};
use pangalactic_path::{AnyDestination, AnySource, PathLayer, StorePath};
use pangalactic_store_dirdb::DirDbStore;
use pangalactic_transfer::Transferor;

#[derive(Debug)]
pub struct StandardStore(Option<Inner>);

type Inner = PathLayer<CidMetaLayer<DirDbStore>>;

pub type StandardPath = StorePath<CidMeta<Hash>>;
pub type StandardAnySource = AnySource<CidMeta<Hash>>;
pub type StandardAnyDestination = AnyDestination<CidMeta<Hash>>;

impl Default for StandardStore {
    fn default() -> Self {
        StandardStore(Some(Inner::default()))
    }
}

impl StandardStore {
    pub async fn transfer(
        &mut self,
        source: StandardAnySource,
        destination: StandardAnyDestination,
    ) -> Result<Option<StandardPath>> {
        self.inner_mut().transfer(source, destination).await
    }

    pub async fn derive(&mut self, plan: StandardPath) -> Result<StandardPath> {
        let inner = self.0.take().unwrap();
        let planlink = inner.resolve_path(&plan).await?;
        let (sdl, attlink) = pangalactic_host::derive(inner.into(), &planlink).await?;
        self.0 = Some(PathLayer::from(sdl));
        let attestation = StandardPath::from(attlink);
        Ok(attestation)
    }

    fn inner_mut(&mut self) -> &mut Inner {
        self.0.as_mut().unwrap()
    }
}
