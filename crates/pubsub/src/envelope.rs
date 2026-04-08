use ed25519_dalek::{Signature, Signer as _, SigningKey};
use pangalactic_serialization::{deserialize, serialize};
use serde::{Deserialize, Serialize};

use crate::{History, SubscribeCap, Subscription};

/// Signed serialized content, the on-wire / on-disk format for a [Subscription]
#[derive(Debug, Serialize, Deserialize)]
pub struct Envelope {
    content: Vec<u8>, // **Performance TODO:** make this `&'de [u8]`
    sig: Signature,
}

impl Envelope {
    /// Sign a new [Subscription] payload, producing an [Envelope] ready for serialization
    pub(crate) fn sign<L>(
        sk: &SigningKey,
        subcap: SubscribeCap,
        history: History<L>,
    ) -> anyhow::Result<Self>
    where
        L: Serialize,
    {
        let content = serialize(&UnverifiedSubscription {
            subcap,
            history: Some(history),
        })?;
        let sig = sk.sign(&content);
        Ok(Envelope { content, sig })
    }
}

/// Isomorphic to [Subscription] but with no claim on a signature or its validity
#[derive(Debug, Serialize, Deserialize)]
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
