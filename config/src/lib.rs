mod pgdirs;

use anyhow::Result;
use serde::de::DeserializeOwned;
use std::path::Path;

pub use self::pgdirs::PgDirs;

/// Load a configuration
pub async fn load<P, C>(subpath: P) -> Result<C>
where
    P: AsRef<Path>,
    C: DeserializeOwned,
{
    PgDirs::singleton().load_config(subpath).await
}
