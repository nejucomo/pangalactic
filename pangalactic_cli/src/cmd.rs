use crate::store::PgStore;
use std::io::Result;
use std::path::Path;

pub fn fs_import(path: &Path) -> Result<()> {
    let _store = dbg!(PgStore::open())?;
    todo!("fs_import({:?})", path);
}

pub fn fs_export(key: String, path: &Path) -> Result<()> {
    todo!("fs_export({:?}, {:?})", key, path);
}
