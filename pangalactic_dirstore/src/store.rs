use pangalactic_store::Store;
use std::io::Result as IOResult;
use std::path::PathBuf;

pub struct DirStore(PathBuf);

impl DirStore {
    pub fn init(datadir: PathBuf) -> DirStore {
        DirStore(datadir)
    }
}

impl Store for DirStore {
    type Key = crate::key::Key;
    type Reader = std::fs::File;
    type Writer = crate::writer::Writer;

    fn open_reader(&self, key: &Self::Key) -> IOResult<Self::Reader> {
        use pangalactic_store::StoreKey;

        std::fs::File::open(self.0.join(key.b64_encode()))
    }

    fn open_writer(&self) -> IOResult<Self::Writer> {
        Self::Writer::open(&self.0)
    }

    fn commit_writer(&mut self, w: Self::Writer) -> IOResult<Self::Key> {
        w.commit()
    }
}
