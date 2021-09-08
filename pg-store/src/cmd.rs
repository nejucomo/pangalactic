use pangalactic_storage::{Link, Storage};
use std::io::Result;
use std::path::Path;

pub fn import(path: &Path) -> Result<Link> {
    let mut store = Storage::open_default()?;
    store.import_path(path)
}

pub fn export(link: &Link, path: &Path) -> Result<()> {
    let store = Storage::open_default()?;
    store.export_path(&link, path)
}

pub fn dump(link: &Link) -> Result<()> {
    use pangalactic_storage::ReadEntry;

    let mut out = std::io::stdout();
    let store = Storage::open_default()?;
    match store.open_entry_reader(&link)? {
        ReadEntry::Dir(d) => d.to_user_json(out),
        ReadEntry::FileStream(mut s) => std::io::copy(&mut s, &mut out).map(|_| ()),
    }
}
