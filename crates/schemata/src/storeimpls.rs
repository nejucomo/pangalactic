use crate::{Attestation, Plan};
use anyhow::Result;
use pangalactic_dir::Directory;
use pangalactic_layer_dir::{LinkDirectory, LinkDirectoryLayer};
use pangalactic_link::Link;
use pangalactic_store::{Commit, Load, Store};

impl<S> Load<LinkDirectoryLayer<S>> for Attestation<Link<S::CID>>
where
    S: Store,
{
    async fn load_from_store(store: &LinkDirectoryLayer<S>, link: &Link<S::CID>) -> Result<Self> {
        let linkdir: LinkDirectory<_> = store.load(link).await?;
        Self::try_from(Directory::from(linkdir))
    }
}

impl<S> Commit<LinkDirectoryLayer<S>> for Attestation<Link<S::CID>>
where
    S: Store,
{
    async fn commit_into_store(self, store: &mut LinkDirectoryLayer<S>) -> Result<Link<S::CID>> {
        store
            .commit(LinkDirectory::from(Directory::from(self)))
            .await
    }
}

impl<S> Load<LinkDirectoryLayer<S>> for Plan<Link<S::CID>>
where
    S: Store,
{
    async fn load_from_store(store: &LinkDirectoryLayer<S>, link: &Link<S::CID>) -> Result<Self> {
        let linkdir: LinkDirectory<_> = store.load(link).await?;
        Self::try_from(Directory::from(linkdir))
    }
}

impl<S> Commit<LinkDirectoryLayer<S>> for Plan<Link<S::CID>>
where
    S: Store,
{
    async fn commit_into_store(self, store: &mut LinkDirectoryLayer<S>) -> Result<Link<S::CID>> {
        store
            .commit(LinkDirectory::from(Directory::from(self)))
            .await
    }
}
