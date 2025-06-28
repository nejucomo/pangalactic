use pangalactic_config::Configuration;
use pangalactic_globset::GlobSet;
use pangalactic_link::Link;
use pangalactic_store::Store;
use serde::{Deserialize, Serialize};

/// The user-scoped tool configuration for revision control
///
/// # Note
///
/// This is contrasted with [WorkspaceConfig] which is a per-workspace configuration.
#[derive(Debug, Serialize, Deserialize)]
pub struct RevConConfig<S>
where
    S: Store,
{
    /// A link to the seed dir distributed w/ this release
    ///
    /// # NOTE
    ///
    /// In the future, the seed directory should be separately versioned and released, and this treated as a dependency of this crate which
    pub seed: Link<S::CID>,
}

impl<S> Configuration for RevConConfig<S>
where
    S: Store,
{
    const NAME: &str = "revcon";
}

/// Workspace-specific configuration, revision controlled by default
#[derive(Debug, Serialize, Deserialize)]
pub struct WorkspaceConfig {
    /// A list of globs to exclude
    pub exclude: GlobSet,
}
