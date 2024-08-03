use async_trait::async_trait;

use crate::{Readable, Store};

#[cfg_attr(not(doc), async_trait)]
pub trait Commit<S>
where
    S: Store,
{
    async fn commit_into_store(self, store: &mut S) -> anyhow::Result<S::CID>;
}

// #[cfg_attr(not(doc), async_trait)]
// impl<S> Commit<S> for S::CID
// where
//     S: Store,
//     S::CID: Clone,
// {
//     async fn commit_into_store(self, _: &mut S) -> anyhow::Result<S::CID> {
//         Ok(self)
//     }
// }

#[cfg_attr(not(doc), async_trait)]
impl<'a, S> Commit<S> for &'a [u8]
where
    S: Store,
{
    async fn commit_into_store(self, store: &mut S) -> anyhow::Result<S::CID> {
        store.commit(Readable(self)).await
    }
}

#[cfg_attr(not(doc), async_trait)]
impl<S> Commit<S> for Vec<u8>
where
    S: Store,
{
    async fn commit_into_store(self, store: &mut S) -> anyhow::Result<S::CID> {
        store.commit(self.as_slice()).await
    }
}
