use pangalactic_globset::GlobSet;
use pangalactic_link::Link;
use pangalactic_name::Name;
use pangalactic_store::Store;
use serde::{Deserialize, Serialize};

#[derive(Debug, Serialize, Deserialize)]
#[serde(bound = "S: Store")]
pub struct Config<S>
where
    S: Store,
{
    pub narrative: Name,
    pub exclude: GlobSet,
    pub head: Option<Link<S::CID>>,
}
