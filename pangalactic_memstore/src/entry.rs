use std::io::{Cursor, Read, Result};
use std::rc::Rc;

#[derive(Clone)]
pub struct Entry(Rc<Cursor<Vec<u8>>>);

impl From<Vec<u8>> for Entry {
    fn from(bytes: Vec<u8>) -> Entry {
        Entry(Rc::new(Cursor::new(bytes)))
    }
}

impl Read for Entry {
    fn read(&mut self, buf: &mut [u8]) -> Result<usize> {
        let cursor = Rc::get_mut(&mut self.0).unwrap();
        cursor.read(buf)
    }
}
