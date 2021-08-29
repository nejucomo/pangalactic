use pangalactic_hashspool::Hash;
use pangalactic_store::Store;
use std::io::Result as IOResult;
use std::path::PathBuf;

pub type Key = Hash;

#[derive(Debug, derive_more::From)]
pub struct DirStore(PathBuf);

impl DirStore {
    pub fn init(datadir: PathBuf) -> DirStore {
        DirStore(datadir)
    }
}

impl Store for DirStore {
    type Key = Key;
    type Reader = std::fs::File;
    type Writer = crate::writer::Writer;

    fn open_reader(&self, key: &Self::Key) -> IOResult<Self::Reader> {
        use pangalactic_codec as codec;
        use pangalactic_fs::file_open;

        let f = file_open(self.0.join(codec::encode_string(&key)))?;
        Ok(f)
    }

    fn open_writer(&self) -> IOResult<Self::Writer> {
        Self::Writer::open(&self.0)
    }

    fn commit_writer(&mut self, w: Self::Writer) -> IOResult<Self::Key> {
        w.commit()
    }
}
