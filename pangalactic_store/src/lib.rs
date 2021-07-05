mod b64;
mod dirstore;
mod key;
mod randtoken;
mod reader;
mod writer;

pub use dirstore::DirStore;

#[cfg(test)]
mod tests {
    use test_case::test_case;

    #[test_case(b"")]
    #[test_case(b"hello world")]
    fn store_then_read(contents: &[u8]) -> std::io::Result<()> {
        use std::io::{Read, Write};
        use testdir::testdir;

        let store = crate::DirStore::init(testdir!());

        let mut w = store.open_writer()?;
        w.write_all(contents)?;
        let key = w.commit()?;

        let mut r = store.open_reader(key)?;
        let mut bytes = vec![];
        r.read_to_end(&mut bytes)?;
        r.verify()?;

        assert_eq!(bytes, contents);
        Ok(())
    }
}
