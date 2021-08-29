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
    ( $name:ident -> $ret:ty : $delegate:expr ) => {
        pub fn $name<P>(path: P) -> Result<$ret>
        where
            P: AsRef<Path> + std::fmt::Debug,
        {
            log::trace!("{}({:?})", stringify!($name), &path);
            $delegate(&path).map_err(PathError::wrap_std(path))
        }
    };
}

macro_rules! wrap_std_fs_canonical {
    ( $name:ident -> $ret:ty ) => {
        wrap_std_fs!($name -> $ret : std::fs::$name);
    };
}

wrap_std_fs!(file_open -> std::fs::File : std::fs::File::open);
wrap_std_fs!(file_create -> std::fs::File : std::fs::File::create);
wrap_std_fs_canonical!(create_dir -> ());
wrap_std_fs_canonical!(read_dir -> std::fs::ReadDir);
