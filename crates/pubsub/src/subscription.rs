use pangalactic_layer_dir::{LinkDirectory, LinkDirectoryLayer};
use pangalactic_link::Link;
use pangalactic_store::{Commit, Store};
use serde::Serialize;

use crate::{History, SubscribeCap};

/// A Successive content produced by a `PublishCap`
///
/// # Note
///
/// Guests should not see subscriptions.
#[derive(Debug)]
pub struct Subscription<C>
where
    C: Serialize,
{
    subcap: SubscribeCap,
    history: Option<History<C>>,
}

impl<C> Subscription<C>
where
    C: Serialize,
{
    pub fn subcap(&self) -> &SubscribeCap {
        &self.subcap
    }

    pub fn history(&self) -> Option<&History<C>> {
        self.history.as_ref()
    }
}

impl<S> Commit<LinkDirectoryLayer<S>> for Subscription<S::CID>
where
    S: Store,
{
    async fn commit_into_store(
        self,
        store: &mut LinkDirectoryLayer<S>,
    ) -> anyhow::Result<Link<S::CID>> {
        let mut ld = LinkDirectory::default();

        let sclink = store.commit(self.subcap).await?;
        ld.insert("subcap", sclink)?;

        if let Some(h) = self.history {
            let histlink = store.commit(h).await?;
            ld.insert("history", histlink)?;
        }

        let link = store.commit(ld).await?;
        Ok(link)
    }
}
