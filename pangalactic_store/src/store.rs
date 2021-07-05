use std::io::Result as IOResult;

pub trait Store: Sized {
    type Key: StoreKey;
    type Reader: ReadVerify;
    type Writer: WriteCommit<Key = Self::Key>;

    fn open_writer(&self) -> IOResult<Self::Writer>;
    fn open_reader(&self, key: &Self::Key) -> IOResult<Self::Reader>;

    fn write(&self, contents: &[u8]) -> IOResult<Self::Key> {
        let w = self.open_writer()?;
        w.write_all_and_commit(contents)
    }

    fn read(&self, key: &Self::Key) -> IOResult<Vec<u8>> {
        let r = self.open_reader(key)?;
        r.read_all_verified()
    }
}

pub trait StoreKey: Eq + serde::Serialize + serde::de::DeserializeOwned {
    fn b64_encode(&self) -> String {
        crate::b64::encode(&serde_cbor::ser::to_vec_packed(&self).unwrap())
    }
}

pub trait ReadVerify: Sized + std::io::Read {
    /// This consumes the reader and must verify that the data previously read correctly matched
    /// the Key. If consumer code neglects to call this, the result may work so long as there are
    /// no corruption issues. Using this method helps applications build resilience against faulty
    /// or malicious Stores. This verification happens awkwardly *after* the entry has been read,
    /// so applications must be ready to rollback any changes based on this data in the case that
    /// verify fails. A helper method reads the entire entry into RAM, then verifies it, then only
    /// returns the data when verify succeeds, allowing for simpler application logic flow at the
    /// expense of RAM and perhaps duplicate reads/memcpys.
    fn verify(self) -> IOResult<()>;

    /// A convenience method which returns a newly allocated Vec with the entry contents only after
    /// it has been verified. Using this method may be less efficient for many applications while
    /// simplifying the case of a corrupted entry.
    fn read_all_verified(mut self) -> IOResult<Vec<u8>> {
        let mut buf = vec![];
        self.read_to_end(&mut buf)?;
        self.verify()?;
        Ok(buf)
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
