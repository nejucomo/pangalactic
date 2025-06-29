use std::fmt::Display;
use std::str::FromStr;

use pangalactic_config::Configuration;
use pangalactic_endpoint::OriginEndpoint;
use pangalactic_globset::GlobSet;
use pangalactic_store::Store;
use serde::{Deserialize, Serialize};

/// App-scoped configuration
#[derive(Debug, Serialize, Deserialize)]
pub struct RevConConfig<S>
where
    S: Store,
    OriginEndpoint<S::CID>: Display + FromStr<Err = anyhow::Error>,
{
    /// A bookkeeping template origin
    pub template: Option<OriginEndpoint<S::CID>>,
}

impl<S> Configuration for RevConConfig<S>
where
    S: Store,
    OriginEndpoint<S::CID>: Display + FromStr<Err = anyhow::Error>,
{
    const NAME: &str = "revcon";
}

/// Workspace-specific configuration, revision controlled by default
#[derive(Debug, Serialize, Deserialize)]
pub struct WorkspaceConfig {
    /// A list of globs to exclude
    pub exclude: GlobSet,
}
