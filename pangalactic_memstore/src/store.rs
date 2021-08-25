use crate::{key::Key, reader::Reader};
use pangalactic_store::Store;
use std::io::Result as IOResult;
use std::rc::Rc;

#[derive(Clone)]
pub struct MemStore(Vec<Rc<Vec<u8>>>);

impl MemStore {
    pub fn new() -> MemStore {
        MemStore(vec![])
    }
}

impl Store for MemStore {
    type Key = Key;
    type Reader = Reader;
    type Writer = Vec<u8>;

    fn open_reader(&self, key: &Self::Key) -> IOResult<Self::Reader> {
        let ix = usize::from(*key);
        let byteref = &self.0[ix];
        Ok(Reader::from(byteref))
    }

    fn open_writer(&self) -> IOResult<Self::Writer> {
        Ok(vec![])
    }

    fn commit_writer(&mut self, w: Vec<u8>) -> IOResult<Key> {
        let key = Key::from(self.0.len());
        self.0.push(Rc::new(w));
        Ok(key)
    }
}
