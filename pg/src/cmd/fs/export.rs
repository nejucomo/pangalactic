use pangalactic_storage::{Dir, Link, Storage};
use std::io::Result;
use std::path::Path;

pub fn export_path(store: &Storage, link: &Link, path: &Path) -> Result<()> {
    use pangalactic_storage::ReadEntry;

    log::debug!("export_path{:?}", (store, link, path));

    match store.open_entry_reader(&link)? {
        ReadEntry::Dir(d) => export_dir(store, path, d),
        ReadEntry::FileStream(s) => export_file(path, s),
    }
}

fn export_dir(store: &Storage, path: &Path, d: Dir) -> Result<()> {
    pangalactic_fs::create_dir(path)?;

    for entry in &d {
        export_path(store, &entry.link, &path.join(&entry.name))?;
    }

    Ok(())
}

fn export_file<R>(path: &Path, mut r: R) -> Result<()>
where
    R: std::io::Read,
{
    let mut f = pangalactic_fs::file_create(path)?;
    std::io::copy(&mut r, &mut f)?;
    Ok(())
}
