use crate::{DirFor, LinkFor, ReadEntry};

use pangalactic_codec as codec;
use pangalactic_node::{Dir, Kind, Link};
use pangalactic_store::{KeyOf, Store, WriterOf};
use std::io::Result as IOResult;
use std::path::Path;

#[derive(Debug, derive_more::From)]
pub struct NodeStore<S>(S);

// Direct / low-level interaction:
impl<S> NodeStore<S>
where
    S: Store,
{
    pub fn open_entry_reader(&self, link: &LinkFor<S>) -> IOResult<ReadEntry<S>> {
        let key = &link.key;
        match link.kind {
            Kind::Dir => self.get_dir(key).map(ReadEntry::Dir),
            Kind::File => self.0.open_reader(key).map(ReadEntry::FileStream),
        }
    }

    pub fn open_file_writer(&self) -> IOResult<WriterOf<S>> {
        self.0.open_writer()
    }

    pub fn commit_file_writer(&mut self, w: WriterOf<S>) -> IOResult<LinkFor<S>> {
        self.commit_writer_kind(w, Kind::File)
    }

    pub fn put_file<B>(&mut self, buf: B) -> IOResult<LinkFor<S>>
    where
        B: AsRef<[u8]>,
    {
        use std::io::Write;

        let mut f = self.open_file_writer()?;
        f.write_all(buf.as_ref())?;
        self.commit_file_writer(f)
    }

    pub fn get_file(&self, key: &KeyOf<S>) -> IOResult<Vec<u8>> {
        self.0.read_bytes(key)
    }

    pub fn put_dir(&mut self, d: &DirFor<S>) -> IOResult<LinkFor<S>> {
        use std::io::Write;

        let bytes = codec::encode_bytes(d);
        let mut w = self.0.open_writer()?;
        w.write_all(&bytes[..])?;
        self.commit_writer_kind(w, Kind::Dir)
    }

    pub fn get_dir(&self, key: &KeyOf<S>) -> IOResult<DirFor<S>> {
        let bytes = self.0.read_bytes(key)?;
        let dir = codec::decode_bytes(&bytes[..]).map_err(|e| {
            use std::io::{Error, ErrorKind::InvalidData};

            Error::new(InvalidData, format!("Could not decode dir link: {:?}", e))
        })?;
        Ok(dir)
    }

    fn commit_writer_kind(&mut self, w: WriterOf<S>, kind: Kind) -> IOResult<LinkFor<S>> {
        let key = self.0.commit_writer(w)?;
        Ok(Link { kind, key })
    }
}

// Filesystem interaction:
impl<S> NodeStore<S>
where
    S: Store + std::fmt::Debug,
{
    pub fn import_path(&mut self, path: &Path) -> IOResult<LinkFor<S>> {
        let link = if path.is_dir() {
            self.import_dir(path)
        } else {
            self.import_file(path)
        }?;
        log::debug!("Stored {:?} <- {:?}", codec::encode_string(&link), path);
        Ok(link)
    }

    pub fn export_path(&self, link: &LinkFor<S>, path: &Path) -> IOResult<()> {
        log::debug!("{:?}.export_path{:?}", self, (link, path));

        match self.open_entry_reader(&link)? {
            ReadEntry::Dir(d) => self.export_dir(path, d),
            ReadEntry::FileStream(s) => export_file(path, s),
        }
    }

    fn import_dir(&mut self, path: &Path) -> IOResult<LinkFor<S>> {
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

    fn import_file(&mut self, path: &Path) -> IOResult<LinkFor<S>> {
        use pangalactic_fs::file_open;

        log::debug!("{:?}.import_file({:?})", self, &path);
        let mut fr = file_open(path)?;
        let mut fw = self.open_file_writer()?;
        std::io::copy(&mut fr, &mut fw)?;
        self.commit_file_writer(fw)
    }

    fn export_dir(&self, path: &Path, d: DirFor<S>) -> IOResult<()> {
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
