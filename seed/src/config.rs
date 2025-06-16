use pangalactic_config::Configuration;
use pangalactic_link::Link;
use pangalactic_store::Store;
use serde::{Deserialize, Serialize};

#[derive(Debug, Serialize, Deserialize)]
pub struct SeedConfig<S>
where
    S: Store,
{
    pub seed_link: Link<S::CID>,
}

impl<S> Configuration for SeedConfig<S>
where
    S: Store,
{
    const NAME: &str = "seed";
}
