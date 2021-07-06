mod key;
mod randtoken;
mod store;
mod writer;

use std::io::Result as IOResult;

pub use store::DirStore;

#[test]
fn test_roundtrip() -> std::io::Result<()> {
    use testdir::testdir;
    pangalactic_store::test_store_then_read_then_store(DirStore::init(testdir!()))
}
