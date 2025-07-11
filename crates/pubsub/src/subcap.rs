use std::fmt::Display;

use ed25519_dalek::{Signature, Verifier, VerifyingKey};
use pangalactic_layer_dir::LinkDirectoryLayer;
use pangalactic_link::Link;
use pangalactic_serialization::b64;
use pangalactic_store::{Commit, Store};
use serde::{Deserialize, Serialize};

const DISPLAY_PREFIX: &str = "pg:subcap:";

#[derive(Debug, Serialize, Deserialize)]
pub struct SubscribeCap {
    vk: VerifyingKey,
}

impl SubscribeCap {
    pub(crate) fn from_vk(vk: VerifyingKey) -> Self {
        SubscribeCap { vk }
    }

    #[allow(dead_code)]
    pub(crate) fn verify(&self, payload: &[u8], sig: &Signature) -> anyhow::Result<()> {
        self.vk.verify(payload, sig)?;
        Ok(())
    }
}

impl Display for SubscribeCap {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        let bstr = b64::serialize(self.vk.as_bytes()).map_err(|_| std::fmt::Error)?;
        write!(f, "{DISPLAY_PREFIX}{bstr}")
    }
}

impl<S> Commit<LinkDirectoryLayer<S>> for SubscribeCap
where
    S: Store,
{
    async fn commit_into_store(
        self,
        store: &mut LinkDirectoryLayer<S>,
    ) -> anyhow::Result<Link<S::CID>> {
        store.commit(self.to_string()).await
    }
}
