use std::io::Result as IOResult;

pub trait Store: Sized {
    type Key: StoreKey;
    type Reader: std::io::Read;
    type Writer: WriteCommit<Key = Self::Key>;

    fn open_writer(&self) -> IOResult<Self::Writer>;
    fn open_reader(&self, key: &Self::Key) -> IOResult<Self::Reader>;

    fn write(&self, contents: &[u8]) -> IOResult<Self::Key> {
        let w = self.open_writer()?;
        w.write_all_and_commit(contents)
    }

    fn read(&self, key: &Self::Key) -> IOResult<Vec<u8>> {
        use std::io::Read;

        let mut buf = vec![];
        let mut r = self.open_reader(key)?;
        r.read_to_end(&mut buf)?;
        Ok(buf)
    }
}

pub trait StoreKey: Eq + serde::Serialize + serde::de::DeserializeOwned {
    fn b64_encode(&self) -> String {
        pangalactic_b64::encode(&self.cbor_encode())
    }

    fn cbor_encode(&self) -> Vec<u8> {
        serde_cbor::ser::to_vec_packed(&self).unwrap()
    }
}

pub trait WriteCommit: Sized + std::io::Write {
    type Key;

    /// This consumes the writer, commits it to the store, and produces the Key for subsequent
    /// reads.
    fn commit(self) -> IOResult<Self::Key>;

    fn write_all_and_commit(mut self, contents: &[u8]) -> IOResult<Self::Key> {
        self.write_all(contents)?;
        self.commit()
    }
}
