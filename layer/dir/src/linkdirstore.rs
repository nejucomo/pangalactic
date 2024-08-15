use std::future::Future;

use anyhow::Result;
use pangalactic_link::Link;
use pangalactic_store::{Commit, Load, Store};

use crate::{LinkDirectory, LinkDirectoryLayer};

pub trait LinkDirectoryStore: Store
where
    LinkDirectory<<Self::InnerStore as Store>::CID>: Commit<Self> + Load<Self>,
{
    type InnerStore: Store;

    fn commit_to_link<T>(
        &mut self,
        object: T,
    ) -> impl Future<Output = Result<Link<<Self::InnerStore as Store>::CID>>> + Send
    where
        T: Commit<LinkDirectoryLayer<Self::InnerStore>> + Send;

    fn load_from_link<T>(
        &self,
        link: &Link<<Self::InnerStore as Store>::CID>,
    ) -> impl Future<Output = Result<T>> + Send
    where
        T: Load<LinkDirectoryLayer<Self::InnerStore>> + Send;
}
