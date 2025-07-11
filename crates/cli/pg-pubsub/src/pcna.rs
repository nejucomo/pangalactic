use std::path::PathBuf;

use anyhow_std::PathAnyhow as _;
use pangalactic_node::Node;
use pangalactic_pubsub::PublishCap;
use pangalactic_serialization::b64;

pub(crate) trait PubCapNodeExt {
    fn with_pubcap_dir(self, pubcap_dir: PathBuf) -> PubCapNodeApp;
}

impl PubCapNodeExt for Node {
    fn with_pubcap_dir(self, pubcap_dir: PathBuf) -> PubCapNodeApp {
        PubCapNodeApp {
            node: self,
            pubcap_dir,
        }
    }
}

pub(crate) struct PubCapNodeApp {
    node: Node,
    pubcap_dir: PathBuf,
}

impl PubCapNodeApp {
    pub(crate) async fn generate(self) -> anyhow::Result<()> {
        let pubcap = PublishCap::generate(rand::rng());
        let subcapstr = pubcap.subscribe_cap().to_string();

        let pcpath = self.pubcap_dir.join(format!("{}.PUBLISH-CAP", &subcapstr));

        let pcbytes = b64::serialize(&pubcap)?;
        pcpath.write_anyhow(pcbytes)?;

        tracing::info!("Wrote {:?}.", pcpath.display());
        todo!(
            "Issue first 'pre-initial' subscription with {:?}",
            self.node
        );
    }

    pub(crate) async fn get_subscribe_cap(self) -> anyhow::Result<()> {
        todo!()
    }
}
