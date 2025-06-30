//! Access a consistent [PathBuf] for app data
use std::path::{Path, PathBuf};

use crate::pgdirs::PgDirs;

/// Get the pangalactic data for `name`
///
/// These names are global across all crates making up the `pg` app, so if any crates reuse the same name they must coordinate carefully.
pub fn get<P>(name: P) -> PathBuf
where
    P: AsRef<Path>,
{
    PgDirs::singleton().data.join(name)
}
