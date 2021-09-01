mod export;
mod import;

use pangalactic_fs::ensure_directory_exists;
use pangalactic_storage::{Link, Storage};
use std::io::Result;
use std::path::Path;

pub fn import(path: &Path) -> Result<Link> {
    let dirs = crate::get_appdirs()?;
    ensure_directory_exists(&dirs.data)?;
    let mut store = Storage::open(dirs.data)?;
    import::import_path(&mut store, path)
}

pub fn export(link: &Link, path: &Path) -> Result<()> {
    let dirs = crate::get_appdirs()?;
    let store = Storage::open(dirs.data)?;
    export::export_path(&store, &link, path)
}

pub fn dump(link: &Link) -> Result<()> {
    let dirs = crate::get_appdirs()?;

    use pangalactic_storage::ReadEntry;

    let mut out = std::io::stdout();
    let store = Storage::open(dirs.data)?;
    match store.open_entry_reader(&link)? {
        ReadEntry::Dir(d) => d.to_user_json(out),
        ReadEntry::FileStream(mut s) => std::io::copy(&mut s, &mut out).map(|_| ()),
    }
}
