use crate::{History, SubscribeCap};

/// A Successive content produced by a `PublishCap`
///
/// # Note
///
/// Guests should not see subscriptions.
#[derive(Debug)]
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
}
