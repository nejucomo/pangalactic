mod import;

use crate::store::{PgLink, PgStore};
use pangalactic_appdirs::AppDirs;
use pangalactic_codec::encode_string;
use pangalactic_fs::ensure_directory_exists;
use std::io::Result;
use std::path::Path;

pub fn import(dirs: AppDirs, path: &Path) -> Result<()> {
    ensure_directory_exists(&dirs.data)?;
    let mut store = PgStore::open(dirs.data)?;
    let link = import::import_path(&mut store, path)?;
    println!("{}", encode_string(&link));
    Ok(())
}

pub fn export(dirs: AppDirs, link: PgLink, path: &Path) -> Result<()> {
    todo!("fs_export{:?}", (dirs, link, path));
}

pub fn dump(dirs: AppDirs, link: PgLink) -> Result<()> {
    use pangalactic_nodestore::ReadEntry::*;

    let mut out = std::io::stdout();
    let store = PgStore::open(dirs.data)?;
    match store.open_entry_reader(&link)? {
        Dir(d) => d.to_user_json(out),
        FileStream(mut s) => std::io::copy(&mut s, &mut out).map(|_| ()),
    }
}
