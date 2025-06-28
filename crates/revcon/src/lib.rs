//! Base layer revision control functionality
#![deny(missing_docs, unsafe_code)]
mod config;
mod workspace;

pub use self::config::{RevConConfig, WorkspaceConfig};
pub use self::workspace::{Workspace, BOOKKEEPING_DIR_NAME};
