use std::fmt::Debug;

use anyhow::Result;
use serde::{de::DeserializeOwned, Serialize};

use crate::PgDirs;

pub trait Configuration: Debug + DeserializeOwned + Serialize {
    const NAME: &str;

    async fn load() -> Result<Self> {
        PgDirs::singleton().load_config(Self::NAME).await
    }

    async fn save(&self) -> Result<()> {
        PgDirs::singleton().write_config(Self::NAME, self).await
    }
}
