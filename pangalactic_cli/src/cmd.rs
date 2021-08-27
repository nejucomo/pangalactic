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
    use pangalactic_errorutil::io_error;
    use pangalactic_nodestore::ReadEntry::*;
    use std::io::ErrorKind::InvalidData;

    let mut out = std::io::stdout();
    let store = PgStore::open(dirs.data)?;
    match store.open_entry_reader(&link)? {
        Dir(d) => serde_json::to_writer_pretty(out, &d)
            .map_err(|e| io_error!(InvalidData, "JSON serialization failure: {:#?}", e)),
        FileStream(mut s) => std::io::copy(&mut s, &mut out).map(|_| ()),
    }
}
