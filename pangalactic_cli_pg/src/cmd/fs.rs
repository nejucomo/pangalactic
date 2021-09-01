mod export;
mod import;

use crate::store::{PgLink, PgStore};
use pangalactic_fs::ensure_directory_exists;
use std::io::Result;
use std::path::Path;

pub fn import(path: &Path) -> Result<PgLink> {
    let dirs = crate::get_appdirs()?;
    ensure_directory_exists(&dirs.data)?;
    let mut store = PgStore::open(dirs.data)?;
    import::import_path(&mut store, path)
}

pub fn export(link: PgLink, path: &Path) -> Result<()> {
    let dirs = crate::get_appdirs()?;
    let store = PgStore::open(dirs.data)?;
    export::export_path(&store, &link, path)
}

pub fn dump(link: PgLink) -> Result<()> {
    let dirs = crate::get_appdirs()?;

    use pangalactic_nodestore::ReadEntry::*;

    let mut out = std::io::stdout();
    let store = PgStore::open(dirs.data)?;
    match store.open_entry_reader(&link)? {
        Dir(d) => d.to_user_json(out),
        FileStream(mut s) => std::io::copy(&mut s, &mut out).map(|_| ()),
    }
}
