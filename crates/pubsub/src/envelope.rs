use ed25519_dalek::Signature;
use pangalactic_serialization::deserialize;
use serde::{Deserialize, Serialize};

use crate::{History, SubscribeCap, Subscription};

/// Signed serialized content used internally
#[derive(Debug, Serialize, Deserialize)]
pub(crate) struct Envelope {
    content: Vec<u8>, // **Performance TODO:** make this `&'de [u8]`
    sig: Signature,
}

/// Isomorphic to [Subscription] but with no claim on a signature or its validity
#[derive(Debug, Deserialize)]
struct UnverifiedSubscription<L> {
    subcap: SubscribeCap,
    history: Option<History<L>>,
}

impl<L> TryFrom<Envelope> for Subscription<L>
where
    L: for<'de> Deserialize<'de>,
{
    type Error = anyhow::Error;

    fn try_from(env: Envelope) -> anyhow::Result<Self> {
        let subslice = env.content.as_slice();
        let uvsub: UnverifiedSubscription<L> = deserialize(subslice)?;
        uvsub.subcap.verify(subslice, &env.sig)?;
        Ok(Subscription::new_verified(uvsub.subcap, uvsub.history))
    }
}
