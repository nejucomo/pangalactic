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
    ( unary $name:ident -> $ret:ty ) => {
        wrap_std_fs!( unary $name -> $ret : std::fs::$name);
    };

    ( binary $name:ident -> $ret:ty ) => {
        wrap_std_fs!( binary $name -> $ret : std::fs::$name);
    };

    ( unary $name:ident -> $ret:ty : $delegate:expr ) => {
        pub fn $name<P>(path: P) -> Result<$ret>
        where
            P: AsRef<Path> + std::fmt::Debug,
        {
            log::trace!("{}({:?})", stringify!($name), &path);
            $delegate(&path).map_err(PathError::wrap_std(path))
        }
    };

    ( binary $name:ident -> $ret:ty : $delegate:expr ) => {
        pub fn $name<P, Q>(p: P, q: Q) -> Result<$ret>
        where
            P: AsRef<Path> + std::fmt::Debug,
            Q: AsRef<Path> + std::fmt::Debug,
        {
            log::trace!("{}{:?}", stringify!($name), (&p, &q));
            $delegate(&p, &q).map_err(PathError::wrap_std2(p, q))
        }
    };
}

wrap_std_fs! {
    unary file_open -> std::fs::File : std::fs::File::open
}

wrap_std_fs! {
    unary file_create -> std::fs::File : std::fs::File::create
}

wrap_std_fs! {
    unary create_dir -> ()
}

wrap_std_fs! {
    unary read_dir -> std::fs::ReadDir
}

wrap_std_fs! {
    binary rename -> ()
}
