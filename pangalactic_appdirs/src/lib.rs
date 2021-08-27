#[macro_export]
macro_rules! pangalactic_appdirs_init {
    () => {
        $crate::AppDirs::init(env!("CARGO_PKG_NAME"))
    };
}

use std::path::PathBuf;

#[derive(Debug)]
pub struct AppDirs {
    pub data: PathBuf,
}

impl AppDirs {
    pub fn init(appname: &str) -> std::io::Result<AppDirs> {
        let ad = AppDirs::new_uncooked(appname)?;
        std::fs::create_dir_all(&ad.data)?;
        Ok(ad)
    }

    fn new_uncooked(appname: &str) -> std::io::Result<AppDirs> {
        use std::io::{Error, ErrorKind::NotFound};

        let mut data = dirs::data_dir().ok_or(Error::new(
            NotFound,
            "Application data dir not defined on this platform.",
        ))?;
        data.push(appname);

        Ok(AppDirs { data })
    }
}
