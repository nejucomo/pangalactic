mod error;

use error::{PathError, Result};
use std::path::Path;

pub fn ensure_directory_exists<P: AsRef<Path>>(dir: P) -> Result<()> {
    use std::io::ErrorKind::AlreadyExists;

    let dirpath: &Path = dir.as_ref();

    match std::fs::create_dir(dirpath) {
        Ok(_) => {
            log::info!("Created directory: {:?}", dirpath);
            Ok(())
        }
        Err(e) => match e.kind() {
            AlreadyExists => {
                log::debug!("Directory {:?} exists.", dirpath);
                Ok(())
            }
            _ => Err(e),
        },
    }
    .map_err(PathError::wrap_std(dirpath))
}

macro_rules! wrap_std_fs {
    ( $name:ident ) => {
        pub fn $name<P>(path: P) -> Result<()>
        where
            P: AsRef<Path> + std::fmt::Debug,
        {
            log::trace!("{}({:?})", stringify!($name), &path);
            std::fs::$name(&path).map_err(PathError::wrap_std(path))
        }
    };
}

wrap_std_fs!(create_dir);
