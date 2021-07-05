use crate::key::Key;
use crate::reader::Reader;
use crate::writer::Writer;
use std::path::PathBuf;

pub struct CHKStore(PathBuf);

impl CHKStore {
    pub fn init(datadir: PathBuf) -> CHKStore {
        CHKStore(datadir)
    }

    pub fn open_writer(&self) -> std::io::Result<Writer> {
        Writer::open(&self.0)
    }

    pub fn open_reader(&self, key: Key) -> std::io::Result<Reader> {
        Reader::open(&self.0, key)
    }
}
