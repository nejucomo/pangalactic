use std::io::Result as IOResult;

pub trait Store: Sized {
    type Key: StoreKey;
    type Reader: std::io::Read;
    type Writer: std::io::Write;

    fn open_reader(&self, key: &Self::Key) -> IOResult<Self::Reader>;
    fn open_writer(&self) -> IOResult<Self::Writer>;
    fn commit_writer(&mut self, w: Self::Writer) -> IOResult<Self::Key>;

    fn write(&mut self, contents: &[u8]) -> IOResult<Self::Key> {
        use std::io::Write;

        let mut w = self.open_writer()?;
        w.write_all(contents)?;
        self.commit_writer(w)
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
