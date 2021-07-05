mod b64;
mod dirstore;
mod randtoken;
mod store;

pub use dirstore::DirStore;
pub use store::{ReadVerify, Store, WriteCommit};

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
    fn store_then_read<S>(store: S, contents: &[u8]) -> std::io::Result<()>
    where
        S: crate::Store,
    {
        let key = store.write(contents)?;
        let bytes = store.read(key)?;
        assert_eq!(bytes, contents);
        Ok(())
    }
}
