use std::io::{Cursor, Read, Result};
use std::rc::Rc;

#[derive(Clone)]
pub struct Reader(Rc<Cursor<Vec<u8>>>);

impl From<Vec<u8>> for Reader {
    fn from(bytes: Vec<u8>) -> Reader {
        Reader(Rc::new(Cursor::new(bytes)))
    }
}

impl Read for Reader {
    fn read(&mut self, buf: &mut [u8]) -> Result<usize> {
        let cursor = Rc::get_mut(&mut self.0).unwrap();
        cursor.read(buf)
    }
}
