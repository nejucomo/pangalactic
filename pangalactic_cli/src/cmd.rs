mod import;

use crate::store::PgStore;
use pangalactic_appdirs::AppDirs;
use pangalactic_fs::ensure_directory_exists;
use std::io::Result;
use std::path::Path;

pub fn fs_import(dirs: AppDirs, path: &Path) -> Result<()> {
    ensure_directory_exists(&dirs.data)?;
    let mut store = dbg!(PgStore::open(dirs.data))?;
    let key = import::import_path(&mut store, path.to_path_buf())?;
    dbg!(key);
    todo!("print key");
}

pub fn fs_export(dirs: AppDirs, key: String, path: &Path) -> Result<()> {
    todo!("fs_export({:?}, {:?}, {:?})", dirs, key, path);
}
