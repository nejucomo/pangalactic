use pangalactic_appdirs::appdirs_init;
use pangalactic_fs::ensure_directory_exists;
use pangalactic_storage::{Link, Storage};
use std::io::Result;
use std::path::Path;

pub fn import(path: &Path) -> Result<Link> {
    let dirs = appdirs_init!()?;
    ensure_directory_exists(&dirs.data)?;
    let mut store = Storage::open(dirs.data)?;
    store.import_path(path)
}

pub fn export(link: &Link, path: &Path) -> Result<()> {
    let dirs = appdirs_init!()?;
    let store = Storage::open(dirs.data)?;
    store.export_path(&link, path)
}

pub fn dump(link: &Link) -> Result<()> {
    use pangalactic_storage::ReadEntry;

    let dirs = appdirs_init!()?;
    let mut out = std::io::stdout();
    let store = Storage::open(dirs.data)?;
    match store.open_entry_reader(&link)? {
        ReadEntry::Dir(d) => d.to_user_json(out),
        ReadEntry::FileStream(mut s) => std::io::copy(&mut s, &mut out).map(|_| ()),
    }
}
