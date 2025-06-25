use std::fmt::Debug;
use std::path::{Path, PathBuf};

use anyhow::Result;
use serde::{de::DeserializeOwned, Serialize};

pub(crate) const APP_NAME: &str = "pangalactic";

/// Manages the config and data directoriesi
///
/// These are named `pangalactic` and located in standardized locations.
#[derive(Debug)]
pub(crate) struct PgDirs {
    pub(crate) data: PathBuf,
    pub(crate) config: PathBuf,
}

impl PgDirs {
    /// Access the singleton instance
    ///
    /// # Panics
    ///
    /// This will panic when first called on systems for which the `dirs` crate does not define standard paths.
    pub(crate) fn singleton() -> &'static Self {
        use once_cell::sync::Lazy;

        static SINGLETON: Lazy<PgDirs> = Lazy::new(|| PgDirs::init().unwrap());
        &SINGLETON
    }

    pub(crate) async fn load_config<P, C>(&self, subpath: P) -> Result<C>
    where
        P: AsRef<Path>,
        C: DeserializeOwned + Debug,
    {
        use tokio::io::AsyncReadExt;

        let path = self.config.join(subpath).with_extension("toml");
        let mut f = tokio::fs::File::open(&path).await?;
        let mut s = String::new();
        f.read_to_string(&mut s).await?;
        let config = toml::from_str(&s)?;
        tracing::debug!("loaded config {:?}: {:#?}", path.display(), &config);
        Ok(config)
    }

    pub(crate) async fn write_config<P, C>(&self, subpath: P, config: &C) -> Result<()>
    where
        P: AsRef<Path>,
        C: Serialize + Debug,
    {
        use tokio::io::AsyncWriteExt;

        let confstr = toml::to_string_pretty(config)?;

        let path = self.config.join(subpath).with_extension("toml");
        tracing::debug!("saving config {:?}: {:#?}", path.display(), &config);
        let mut f = tokio::fs::File::create(path).await?;
        f.write_all(confstr.as_bytes()).await?;
        Ok(())
    }

    fn init() -> Result<Self> {
        macro_rules! get_dir {
            ( $d:ident ) => {
                dirs::$d().map(|d| d.join(APP_NAME)).ok_or_else(|| {
                    anyhow::anyhow!("platform error: `dirs::{}` undefined", stringify!($d))
                })
            };
        }

        Ok(PgDirs {
            data: get_dir!(data_local_dir)?,
            config: get_dir!(config_dir)?,
        })
    }
}
