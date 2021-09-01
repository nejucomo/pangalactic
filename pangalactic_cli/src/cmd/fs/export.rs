use crate::store::{PgKey, PgLink, PgStore};
use pangalactic_node::Dir;
use std::io::Result;
use std::path::Path;

pub fn export_path(store: &PgStore, link: &PgLink, path: &Path) -> Result<()> {
    use pangalactic_nodestore::ReadEntry::*;

    match store.open_entry_reader(&link)? {
        Dir(d) => export_dir(store, path, d),
        FileStream(s) => export_file(path, s),
    }
}

fn export_dir(store: &PgStore, path: &Path, d: Dir<PgKey>) -> Result<()> {
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
