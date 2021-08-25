use crate::{entry::Entry, key::Key, writer::Writer};
use pangalactic_store::Store;
use std::io::Result as IOResult;
use std::rc::Rc;

#[derive(Clone)]
pub struct MemStore(Rc<Vec<Entry>>);

impl MemStore {
    pub fn new() -> MemStore {
        MemStore(Rc::new(vec![]))
    }

    pub(crate) fn add_entry(&mut self, bytes: Vec<u8>) -> Key {
        let entries = Rc::get_mut(&mut self.0).unwrap();
        let key = Key::from(entries.len());
        entries.push(Entry::from(bytes));
        key
    }
}

impl Store for MemStore {
    type Key = Key;
    type Reader = Entry;
    type Writer = Writer;

    fn open_writer(&self) -> IOResult<Self::Writer> {
        Ok(Self::Writer::new(self.clone()))
    }

    fn open_reader(&self, key: &Self::Key) -> IOResult<Self::Reader> {
        let ix = usize::from(*key);
        let entries = self.0.as_ref();
        let entry = &entries[ix];
        Ok(entry.clone())
    }
}
