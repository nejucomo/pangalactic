use anyhow::Result;
use pangalactic_iowrappers::{Readable, Writable};
use pangalactic_layer_cidmeta::CidMetaLayer;
use pangalactic_layer_storedir::StoreDirectoryLayer;
use pangalactic_store::{Commit, Load, Store};

use crate::{inner, HostAnyDestination, HostAnySource, HostStorePath};

#[derive(Debug)]
pub struct HostLayer<S>(Option<inner::Layer<S>>)
where
    S: Store;

impl<S> Default for HostLayer<S>
where
    S: Store + Default,
{
    fn default() -> Self {
        HostLayer(Some(inner::Layer::default()))
    }
}

impl<S> Store for HostLayer<S>
where
    S: Store,
{
    type CID = HostStorePath<S>;
    type Reader = Readable<inner::Reader<S>>;
    type Writer = Writable<inner::Writer<S>>;

    async fn open_writer(&self) -> Result<Self::Writer> {
        self.inner_ref().open_writer().await.map(Writable)
    }
}

impl<S> Commit<HostLayer<S>> for Writable<inner::Writer<S>>
where
    S: Store,
{
    async fn commit_into_store(
        self,
        store: &mut HostLayer<S>,
    ) -> Result<<HostLayer<S> as Store>::CID> {
        self.0.commit_into_store(store.inner_mut()).await
    }
}

impl<S> Load<HostLayer<S>> for Readable<inner::Reader<S>>
where
    S: Store,
{
    async fn load_from_store(store: &HostLayer<S>, cid: &HostStorePath<S>) -> Result<Self> {
        inner::Reader::<S>::load_from_store(store.inner_ref(), cid)
            .await
            .map(Readable)
    }
}

impl<S> HostLayer<S>
where
    S: Store,
{
    pub async fn transfer(
        &mut self,
        source: HostAnySource<S>,
        destination: HostAnyDestination<S>,
    ) -> Result<Option<HostStorePath<S>>> {
        self.inner_mut().transfer(source, destination).await
    }

    pub async fn derive(&mut self, plan: HostStorePath<S>) -> Result<HostStorePath<S>> {
        let inner = self.0.take().check_inner_invariant();
        let planlink = inner.resolve_path(&plan).await?;
        let (sdl, attlink) = pangalactic_host::derive(inner.into(), &planlink).await?;
        self.0 = Some(inner::Layer::from(sdl));
        let attestation = HostStorePath::<S>::from(attlink);
        Ok(attestation)
    }

    pub fn storedir_ref(&self) -> &StoreDirectoryLayer<CidMetaLayer<S>> {
        self.inner_ref().as_ref()
    }

    pub fn storedir_mut(&mut self) -> &mut StoreDirectoryLayer<CidMetaLayer<S>> {
        self.inner_mut().as_mut()
    }

    fn inner_ref(&self) -> &inner::Layer<S> {
        self.0.as_ref().check_inner_invariant()
    }

    fn inner_mut(&mut self) -> &mut inner::Layer<S> {
        self.0.as_mut().check_inner_invariant()
    }
}

trait InnerInvariant<T> {
    fn check_inner_invariant(self) -> T;
}

impl<T> InnerInvariant<T> for Option<T> {
    fn check_inner_invariant(self) -> T {
        self.expect("inner-always-some invariant violated")
    }
}
