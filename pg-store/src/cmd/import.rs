use crate::store::{PgLink, PgStore};
use std::io::Result;
use std::path::Path;

pub fn import_path(store: &mut PgStore, path: &Path) -> Result<PgLink> {
    use pangalactic_codec::encode_string;

    let link = if path.is_dir() {
        import_dir(store, path)
    } else {
        import_file(store, path)
    }?;
    log::debug!("Stored {:?} <- {:?}", encode_string(&link), path);
    Ok(link)
}

pub fn import_dir(store: &mut PgStore, path: &Path) -> Result<PgLink> {
    use pangalactic_fs::read_dir;
    use pangalactic_node::{Dir, Entry};

    log::debug!("import_dir{:?}", (&store, path));
    let mut dirnode = Dir::new();
    for entryres in read_dir(path)? {
        let subpath = &entryres?.path();
        let name = get_path_name(subpath)?;
        let link = import_path(store, subpath)?;
        dirnode.push_entry(Entry { name, link });
    }
    store.put_dir(&dirnode)
}

pub fn import_file(store: &mut PgStore, path: &Path) -> Result<PgLink> {
    use pangalactic_fs::file_open;

    log::debug!("import_file{:?}", (&store, &path));
    let mut fr = file_open(path)?;
    let mut fw = store.open_file_writer()?;
    std::io::copy(&mut fr, &mut fw)?;
    store.commit_file_writer(fw)
}

fn get_path_name(path: &Path) -> Result<String> {
    use pangalactic_errorutil::ok_or_io_error;

    let os = ok_or_io_error!(
        path.file_name(),
        std::io::ErrorKind::NotFound,
        "Path {:?} has no file name.",
        path
    )?;
    let s = ok_or_io_error!(
        os.to_str(),
        std::io::ErrorKind::InvalidData,
        "Path {:?} has non-UTF8 name.",
        path
    )?;
    Ok(String::from(s))
}
