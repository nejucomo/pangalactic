use anyhow::Result;
use serde::de::DeserializeOwned;
use std::path::{Path, PathBuf};

pub const APP_NAME: &str = "pangalactic";

#[derive(Debug)]
pub struct PgDirs {
    pub data: PathBuf,
    pub config: PathBuf,
}

impl PgDirs {
    pub fn singleton() -> &'static Self {
        use once_cell::sync::Lazy;

        static SINGLETON: Lazy<PgDirs> = Lazy::new(|| {
            let pd = PgDirs::init().unwrap();
            tracing::debug!("initialized {:#?}", &pd);
            pd
        });

        &SINGLETON
    }

    pub async fn load_config<P, C>(&self, subpath: P) -> Result<C>
    where
        P: AsRef<Path>,
        C: DeserializeOwned,
    {
        use tokio::io::AsyncReadExt;

        let path = self.config.join(subpath).with_extension("toml");
        let mut f = tokio::fs::File::open(path).await?;
        let mut s = String::new();
        f.read_to_string(&mut s).await?;
        let config = toml::from_str(&s)?;
        Ok(config)
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
