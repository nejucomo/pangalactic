mod b64;
mod dirstore;
mod randtoken;
mod store;

pub use dirstore::DirStore;
pub use store::{ReadVerify, Store, StoreKey, WriteCommit};

#[cfg(test)]
mod tests {
    use crate::DirStore;
    use test_case::test_case;

    fn make_dirstore() -> DirStore {
        use testdir::testdir;
        crate::DirStore::init(testdir!())
    }

    #[test_case(make_dirstore(), b"")]
    #[test_case(make_dirstore(), b"hello world")]
    fn store_then_read_then_store<S>(store: S, contents: &[u8]) -> std::io::Result<()>
    where
        S: crate::Store,
    {
        use crate::StoreKey;

        let key = store.write(contents)?;
        let c2 = store.read(&key)?;
        assert_eq!(c2, contents);
        let k2 = store.write(contents)?;
        assert!(k2 == key, "{:?} != {:?}", k2.b64_encode(), key.b64_encode());
        Ok(())
    }
}
