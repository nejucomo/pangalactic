use crate::{Attestation, Plan};
use anyhow::Result;
use pangalactic_dir::Directory;
use pangalactic_layer_dir::{LinkDirectory, LinkDirectoryStore};
use pangalactic_link::Link;
use pangalactic_store::{Commit, Load, Store};

impl<S> Load<S> for Attestation<Link<<S::InnerStore as Store>::CID>>
where
    S: LinkDirectoryStore,
    LinkDirectory<<S::InnerStore as Store>::CID>: Commit<S> + Load<S>,
{
    async fn load_from_store(store: &S, link: &S::CID) -> Result<Self> {
        let linkdir: LinkDirectory<_> = store.load(link).await?;
        Self::try_from(Directory::from(linkdir))
    }
}

impl<S> Commit<S> for Attestation<Link<<S::InnerStore as Store>::CID>>
where
    S: LinkDirectoryStore,
    LinkDirectory<<S::InnerStore as Store>::CID>: Commit<S> + Load<S>,
{
    async fn commit_into_store(self, store: &mut S) -> Result<S::CID> {
        store
            .commit(LinkDirectory::from(Directory::from(self)))
            .await
    }
}

impl<S> Load<S> for Plan<Link<<S::InnerStore as Store>::CID>>
where
    S: LinkDirectoryStore,
    LinkDirectory<<S::InnerStore as Store>::CID>: Commit<S> + Load<S>,
{
    async fn load_from_store(store: &S, link: &S::CID) -> Result<Self> {
        let linkdir: LinkDirectory<_> = store.load(link).await?;
        Self::try_from(Directory::from(linkdir))
    }
}

impl<S> Commit<S> for Plan<Link<<S::InnerStore as Store>::CID>>
where
    S: LinkDirectoryStore,
    LinkDirectory<<S::InnerStore as Store>::CID>: Commit<S> + Load<S>,
{
    async fn commit_into_store(self, store: &mut S) -> Result<S::CID> {
        store
            .commit(LinkDirectory::from(Directory::from(self)))
            .await
    }
}
