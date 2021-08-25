use crate::key::Key;
use crate::MemStore;
use pangalactic_store::WriteCommit;
use std::io::{Result, Write};

pub struct Writer {
    store: MemStore,
    bytes: Vec<u8>,
}

impl Writer {
    pub(crate) fn new(store: MemStore) -> Writer {
        let bytes = vec![];
        Writer { store, bytes }
    }
}

impl Write for Writer {
    fn write(&mut self, buf: &[u8]) -> Result<usize> {
        self.bytes.write(buf)
    }

    fn flush(&mut self) -> Result<()> {
        Ok(())
    }
}

impl WriteCommit for Writer {
    type Key = Key;

    fn commit(mut self) -> Result<Self::Key> {
        Ok(self.store.add_entry(self.bytes))
    }
}
