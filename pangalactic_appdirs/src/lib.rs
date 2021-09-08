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
    pub fn new(pkgname: &str) -> std::io::Result<AppDirs> {
        use std::io::{Error, ErrorKind::NotFound};

        if let Some(subname) = pkgname.strip_prefix("pangalactic_") {
            let mut data = dirs::data_dir().ok_or(Error::new(
                NotFound,
                "Application data dir not defined on this platform.",
            ))?;
            data.push("pangalactic");
            pangalactic_fs::ensure_directory_exists(&data)?;

            data.push(subname);

            Ok(AppDirs { data })
        } else {
            panic!(
                "AppDirs::new({:?}) must be provided with a `pangalactic_*` package name.",
                pkgname
            );
        }
    }
}
