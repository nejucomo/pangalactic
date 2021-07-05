use crate::reader::Reader;
use crate::writer::Writer;
use crate::Store;
use std::path::PathBuf;

pub struct DirStore(PathBuf);

impl DirStore {
    pub fn init(datadir: PathBuf) -> DirStore {
        DirStore(datadir)
    }
}

impl Store for DirStore {
    type Key = crate::key::Key;
    type Reader = crate::reader::Reader;
    type Writer = crate::writer::Writer;

    fn open_writer(&self) -> std::io::Result<Writer> {
        Writer::open(&self.0)
    }

    fn open_reader(&self, key: Self::Key) -> std::io::Result<Reader> {
        Reader::open(&self.0, key)
    }
}
