use anyhow::Result;
use std::path::PathBuf;

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
