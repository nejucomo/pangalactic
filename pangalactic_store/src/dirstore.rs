mod key;
mod reader;
mod writer;

use crate::Store;
use std::io::Result as IOResult;
use std::path::PathBuf;

pub struct DirStore(PathBuf);

impl DirStore {
    pub fn init(datadir: PathBuf) -> DirStore {
        DirStore(datadir)
    }
}

impl Store for DirStore {
    type Key = self::key::Key;
    type Reader = self::reader::Reader;
    type Writer = self::writer::Writer;

    fn open_writer(&self) -> IOResult<Self::Writer> {
        Self::Writer::open(&self.0)
    }

    fn open_reader(&self, key: Self::Key) -> IOResult<Self::Reader> {
        Self::Reader::open(&self.0, key)
    }
}
