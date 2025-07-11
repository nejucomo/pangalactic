use ed25519_dalek::Signature;
use pangalactic_layer_dir::{LinkDirectory, LinkDirectoryLayer};
use pangalactic_link::Link;
use pangalactic_serialization::b64;
use pangalactic_store::{Commit, Store};
use serde::Serialize;

use crate::Subscription;

/// A Successive content produced by a `PublishCap`
///
/// # Note
///
/// This should be transparent to guests.
#[derive(Debug)]
pub struct SubscriptionEnvelope<C>
where
    C: Serialize,
{
    subscription: Subscription<C>,
    /// Invariant: The signature is valid over the serialization of the link created by `store.commit(self.subscription)`
    signature: Signature,
}

impl<S> Commit<LinkDirectoryLayer<S>> for SubscriptionEnvelope<S::CID>
where
    S: Store,
{
    async fn commit_into_store(
        self,
        store: &mut LinkDirectoryLayer<S>,
    ) -> anyhow::Result<Link<S::CID>> {
        let subscriptionlink = store.commit(self.subscription).await?;
        let sigstr = b64::serialize(&self.signature)?;
        let signaturelink = store.commit(sigstr).await?;

        let mut ld = LinkDirectory::default();
        ld.insert("subscription", subscriptionlink)?;
        ld.insert("signature", signaturelink)?;

        let link = store.commit(ld).await?;
        Ok(link)
    }
}
