use std::io::{Read, Result};
use std::rc::Rc;

#[derive(Clone)]
pub struct Reader {
    byteref: Rc<Vec<u8>>,
    position: usize,
}

impl<'a> From<&'a Rc<Vec<u8>>> for Reader {
    fn from(byteref: &Rc<Vec<u8>>) -> Reader {
        Reader {
            byteref: byteref.clone(),
            position: 0,
        }
    }
}

impl Read for Reader {
    fn read(&mut self, buf: &mut [u8]) -> Result<usize> {
        let bytesleft = &self.byteref[self.position..];
        let n = std::cmp::min(bytesleft.len(), buf.len());
        buf[..n].copy_from_slice(&bytesleft[..n]);
        self.position += n;
        Ok(n)
    }
}
