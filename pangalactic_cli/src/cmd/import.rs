use crate::store::{PgLink, PgStore};
use std::io::Result;
use std::path::PathBuf;

pub fn import_path(store: &mut PgStore, path: PathBuf) -> Result<PgLink> {
    if path.is_dir() {
        import_dir(store, path)
    } else {
        import_file(store, path)
    }
}

pub fn import_dir(store: &mut PgStore, path: PathBuf) -> Result<PgLink> {
    todo!("import_dir{:?}", (store, path));
}

pub fn import_file(store: &mut PgStore, path: PathBuf) -> Result<PgLink> {
    log::debug!("import_file{:?}", (&store, &path));
    let mut fr = std::fs::File::open(path)?;
    let mut fw = store.open_file_writer()?;
    std::io::copy(&mut fr, &mut fw)?;
    store.commit_file_writer(fw)
}
