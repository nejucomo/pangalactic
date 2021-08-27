mod import;

use crate::store::{PgLink, PgStore};
use pangalactic_appdirs::AppDirs;
use pangalactic_codec::encode_string;
use pangalactic_fs::ensure_directory_exists;
use std::io::Result;
use std::path::Path;

pub fn fs_import(dirs: AppDirs, path: &Path) -> Result<()> {
    ensure_directory_exists(&dirs.data)?;
    let mut store = PgStore::open(dirs.data)?;
    let link = import::import_path(&mut store, path)?;
    println!("{}", encode_string(&link));
    Ok(())
}

pub fn fs_export(dirs: AppDirs, link: PgLink, path: &Path) -> Result<()> {
    todo!("fs_export{:?}", (dirs, link, path));
}

pub fn fs_dump(dirs: AppDirs, link: PgLink) -> Result<()> {
    use pangalactic_nodestore::ReadEntry::*;

    let store = PgStore::open(dirs.data)?;
    match store.open_entry_reader(&link)? {
        Dir(d) => todo!("fs_dump dir {:?}", d),
        FileStream(mut s) => std::io::copy(&mut s, &mut std::io::stdout()).map(|_| ()),
    }
}
