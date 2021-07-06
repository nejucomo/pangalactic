mod key;
mod randtoken;
mod writer;

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
    type Key = self::key::Key;
    type Reader = std::fs::File;
    type Writer = self::writer::Writer;

    fn open_writer(&self) -> IOResult<Self::Writer> {
        Self::Writer::open(&self.0)
    }

    fn open_reader(&self, key: &Self::Key) -> IOResult<Self::Reader> {
        use pangalactic_store::StoreKey;

        std::fs::File::open(self.0.join(key.b64_encode()))
    }
}

#[test]
fn test_roundtrip() -> std::io::Result<()> {
    use testdir::testdir;
    pangalactic_store::test_store_then_read_then_store(DirStore::init(testdir!()))
}
