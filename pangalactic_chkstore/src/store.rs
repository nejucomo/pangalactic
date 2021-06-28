use crate::writer::Writer;
use std::path::PathBuf;

pub struct CHKStore(PathBuf);

impl CHKStore {
    pub fn init() -> Result<CHKStore, pangalactic_dirs::InitError> {
        let datadir = pangalactic_dirs::datadir!()?;
        Ok(CHKStore(datadir))
    }

    pub fn open_writer(&self) -> std::io::Result<Writer> {
        Writer::open(&self.0)
    }
}
