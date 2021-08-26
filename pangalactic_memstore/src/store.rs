use crate::{reader::Reader, writer::Writer};
use pangalactic_hashspool::Hash;
use pangalactic_store::Store;
use std::collections::HashMap;
use std::io::Result as IOResult;
use std::rc::Rc;

#[derive(Clone)]
pub struct MemStore(HashMap<Hash, Rc<Vec<u8>>>);

impl MemStore {
    pub fn new() -> MemStore {
        MemStore(HashMap::new())
    }
}

impl Store for MemStore {
    type Key = Hash;
    type Reader = Reader;
    type Writer = Writer;

    fn open_reader(&self, key: &Self::Key) -> IOResult<Self::Reader> {
        use std::io::{Error, ErrorKind::NotFound};

        match self.0.get(key) {
            Some(byteref) => Ok(Reader::from(byteref)),
            None => Err(Error::new(NotFound, format!("Key {:?}", key))),
        }
    }

    fn open_writer(&self) -> IOResult<Self::Writer> {
        Ok(Writer::new())
    }

    fn commit_writer(&mut self, w: Writer) -> IOResult<Self::Key> {
        let (key, bytes) = w.finish();
        self.0.insert(key, Rc::new(bytes));
        Ok(key)
    }
}
