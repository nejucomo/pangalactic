use crate::FileWriter;
use crate::ReadEntry;
use crate::Traverse;

use pangalactic_codec as codec;
use pangalactic_node::{Dir, Kind, Link};
use pangalactic_store::Store;
use std::io::Result as IOResult;
use std::path::Path;

#[derive(Debug, derive_more::From)]
pub struct NodeStore<S>(S);

// Direct / low-level interaction:
impl<S> NodeStore<S>
where
    S: Store,
{
    pub fn open_entry_reader(&self, link: &Link<<S as Store>::Key>) -> IOResult<ReadEntry<S>> {
        let key = &link.key;
        match link.kind {
            Kind::Dir => self.get_dir(key).map(ReadEntry::Dir),
            Kind::File => self.0.open_reader(key).map(ReadEntry::FileStream),
        }
    }

    pub fn open_file_writer(&self) -> IOResult<FileWriter<<S as Store>::Writer>> {
        self.0.open_writer().map(FileWriter::from)
    }

    pub fn commit_file_writer(
        &mut self,
        w: FileWriter<<S as Store>::Writer>,
    ) -> IOResult<Link<<S as Store>::Key>> {
        self.commit_writer_kind(w.unwrap(), Kind::File)
    }

    pub fn traverse<'a>(&'a self, link: &Link<<S as Store>::Key>) -> Traverse<'a, S> {
        let mylink: Link<<S as Store>::Key> = link.clone();

        Traverse::new(self, mylink)
    }

    pub fn put_dir(&mut self, d: &Dir<<S as Store>::Key>) -> IOResult<Link<<S as Store>::Key>> {
        use std::io::Write;

        let bytes = codec::encode_bytes(d);
        let mut w = self.0.open_writer()?;
        w.write_all(&bytes[..])?;
        self.commit_writer_kind(w, Kind::Dir)
    }

    pub fn get_dir(&self, key: &<S as Store>::Key) -> IOResult<Dir<<S as Store>::Key>> {
        let bytes = self.0.read_bytes(key)?;
        let dir = codec::decode_bytes(&bytes[..]).map_err(|e| {
            use std::io::{Error, ErrorKind::InvalidData};

            Error::new(InvalidData, format!("Could not decode dir link: {:?}", e))
        })?;
        Ok(dir)
    }

    fn commit_writer_kind(
        &mut self,
        w: <S as Store>::Writer,
        kind: Kind,
    ) -> IOResult<Link<<S as Store>::Key>> {
        let key = self.0.commit_writer(w)?;
        Ok(Link { kind, key })
    }
}

// Filesystem interaction:
impl<S> NodeStore<S>
where
    S: Store + std::fmt::Debug,
{
    pub fn import_path(&mut self, path: &Path) -> IOResult<Link<<S as Store>::Key>> {
        let link = if path.is_dir() {
            self.import_dir(path)
        } else {
            self.import_file(path)
        }?;
        log::debug!("Stored {:?} <- {:?}", codec::encode_string(&link), path);
        Ok(link)
    }

    pub fn export_path(&self, link: &Link<<S as Store>::Key>, path: &Path) -> IOResult<()> {
        log::debug!("{:?}.export_path{:?}", self, (link, path));

        match self.open_entry_reader(&link)? {
            ReadEntry::Dir(d) => self.export_dir(path, d),
            ReadEntry::FileStream(s) => export_file(path, s),
        }
    }

    fn import_dir(&mut self, path: &Path) -> IOResult<Link<<S as Store>::Key>> {
        use pangalactic_fs::read_dir;
        use pangalactic_node::Entry;

        log::debug!("{:?}.import_dir({:?})", self, path);
        let mut dirnode = Dir::new();
        for entryres in read_dir(path)? {
            let subpath = &entryres?.path();
            let name = get_path_name(subpath)?;
            let link = self.import_path(subpath)?;
            dirnode.push_entry(Entry { name, link });
        }
        self.put_dir(&dirnode)
    }

    fn import_file(&mut self, path: &Path) -> IOResult<Link<<S as Store>::Key>> {
        use pangalactic_fs::file_open;

        log::debug!("{:?}.import_file({:?})", self, &path);
        let mut fr = file_open(path)?;
        let mut fw = self.open_file_writer()?;
        std::io::copy(&mut fr, &mut fw)?;
        self.commit_file_writer(fw)
    }

    fn export_dir(&self, path: &Path, d: Dir<<S as Store>::Key>) -> IOResult<()> {
        pangalactic_fs::create_dir(path)?;

        for entry in &d {
            self.export_path(&entry.link, &path.join(&entry.name))?;
        }

        Ok(())
    }
}

fn export_file<R>(path: &Path, mut r: R) -> IOResult<()>
where
    R: std::io::Read,
{
    let mut f = pangalactic_fs::file_create(path)?;
    std::io::copy(&mut r, &mut f)?;
    Ok(())
}

fn get_path_name(path: &Path) -> IOResult<String> {
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
