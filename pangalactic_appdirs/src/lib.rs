#[macro_export]
macro_rules! appdirs_init {
    () => {
        $crate::AppDirs::new(env!("CARGO_PKG_NAME"))
    };
}

use std::path::PathBuf;

#[derive(Debug)]
pub struct AppDirs {
    pub data: PathBuf,
}

impl AppDirs {
    pub fn new(appname: &str) -> std::io::Result<AppDirs> {
        use std::io::{Error, ErrorKind::NotFound};

        let mut data = dirs::data_dir().ok_or(Error::new(
            NotFound,
            "Application data dir not defined on this platform.",
        ))?;
        data.push(appname);

        Ok(AppDirs { data })
    }
}
