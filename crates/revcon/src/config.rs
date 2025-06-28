use pangalactic_globset::GlobSet;
use serde::{Deserialize, Serialize};

/// Workspace-specific configuration, revision controlled by default
#[derive(Debug, Serialize, Deserialize)]
pub struct WorkspaceConfig {
    /// A list of globs to exclude
    pub exclude: GlobSet,
}
