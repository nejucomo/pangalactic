use serde::de::DeserializeOwned;
use serde::Deserialize;

use crate::envelope::Envelope;
use crate::{History, SubscribeCap};

/// A Successive content produced by a `PublishCap`
///
/// # Note
///
/// Guests should not see subscriptions.
#[derive(Debug, Deserialize)]
#[serde(try_from = "Envelope", bound(deserialize = "L: DeserializeOwned"))]
pub struct Subscription<L> {
    subcap: SubscribeCap,
    history: Option<History<L>>,
}

impl<L> Subscription<L> {
    pub fn subcap(&self) -> &SubscribeCap {
        &self.subcap
    }

    pub fn history(&self) -> Option<&History<L>> {
        self.history.as_ref()
    }

    /// # Precondition
    ///
    /// The caller guarantees these fields are verified to come from `subcap`'s associated `pubcap`.
    pub(crate) fn new_verified(subcap: SubscribeCap, history: Option<History<L>>) -> Self {
        Subscription { subcap, history }
    }
}
