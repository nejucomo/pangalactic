use crate::store::{Key, PgStore};
use std::io::Result;
use std::path::PathBuf;

pub fn import_path(store: &mut PgStore, path: PathBuf) -> Result<Key> {
    if path.is_dir() {
        import_dir(store, path)
    } else {
        import_file(store, path)
    }
}

pub fn import_dir(store: &mut PgStore, path: PathBuf) -> Result<Key> {
    todo!("import_dir{:?}", (store, path));
}

pub fn import_file(store: &mut PgStore, path: PathBuf) -> Result<Key> {
    todo!("import_dir{:?}", (store, path));
}
