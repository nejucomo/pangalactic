use std::fmt::Debug;

use anyhow::Result;
use serde::{de::DeserializeOwned, Serialize};

use crate::pgdirs::PgDirs;

/// A configuration state mapped directly to a config file
pub trait Configuration: Debug + DeserializeOwned + Serialize {
    /// The configuration file is named `<NAME>.toml` inside the [PgDirs] config dir
    const NAME: &str;

    /// The load and deserialize the config file
    async fn load() -> Result<Self> {
        PgDirs::singleton().load_config(Self::NAME).await
    }

    /// The serialize and save the config file
    async fn save(&self) -> Result<()> {
        PgDirs::singleton().write_config(Self::NAME, self).await
    }
}
