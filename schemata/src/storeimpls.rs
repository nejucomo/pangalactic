use crate::{Attestation, Plan};
use anyhow::Result;
use pangalactic_dir::Directory;
use pangalactic_layer_dir::{StoreDirectory, StoreDirectoryLayer};
use pangalactic_link::Link;
use pangalactic_store::{Commit, Load, Store};

impl<S> Load<StoreDirectoryLayer<S>> for Attestation<Link<S::CID>>
where
    S: Store,
{
    async fn load_from_store(store: &StoreDirectoryLayer<S>, link: &Link<S::CID>) -> Result<Self> {
        let storedir: StoreDirectory<_> = store.load(link).await?;
        Self::try_from(Directory::from(storedir))
    }
}

impl<S> Commit<StoreDirectoryLayer<S>> for Attestation<Link<S::CID>>
where
    S: Store,
{
    async fn commit_into_store(self, store: &mut StoreDirectoryLayer<S>) -> Result<Link<S::CID>> {
        store
            .commit(StoreDirectory::from(Directory::from(self)))
            .await
    }
}

impl<S> Load<StoreDirectoryLayer<S>> for Plan<Link<S::CID>>
where
    S: Store,
{
    async fn load_from_store(store: &StoreDirectoryLayer<S>, link: &Link<S::CID>) -> Result<Self> {
        let storedir: StoreDirectory<_> = store.load(link).await?;
        Self::try_from(Directory::from(storedir))
    }
}

impl<S> Commit<StoreDirectoryLayer<S>> for Plan<Link<S::CID>>
where
    S: Store,
{
    async fn commit_into_store(self, store: &mut StoreDirectoryLayer<S>) -> Result<Link<S::CID>> {
        store
            .commit(StoreDirectory::from(Directory::from(self)))
            .await
    }
}
