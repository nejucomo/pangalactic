use serde::{de::DeserializeOwned, Serialize};
use std::io::Result as IOResult;

pub trait Store: Sized {
    type Key: Eq + Clone + std::fmt::Debug + DeserializeOwned + Serialize;
    type Reader: std::io::Read;
    type Writer: std::io::Write;

    fn open_reader(&self, key: &Self::Key) -> IOResult<Self::Reader>;
    fn open_writer(&self) -> IOResult<Self::Writer>;
    fn commit_writer(&mut self, w: Self::Writer) -> IOResult<Self::Key>;

    fn write_bytes(&mut self, contents: &[u8]) -> IOResult<Self::Key> {
        use std::io::Write;

        let mut w = self.open_writer()?;
        w.write_all(contents)?;
        self.commit_writer(w)
    }

    fn read_bytes(&self, key: &Self::Key) -> IOResult<Vec<u8>> {
        use std::io::Read;

        let mut buf = vec![];
        let mut r = self.open_reader(key)?;
        r.read_to_end(&mut buf)?;
        Ok(buf)
    }
}
