use crate::key::Key;
use pangalactic_hashspool::HashSpool;
use std::io::{Result, Write};

pub struct Writer(HashSpool<Vec<u8>>);

impl Writer {
    pub(crate) fn new() -> Writer {
        Writer(HashSpool::new(vec![]))
    }

    pub(crate) fn finish(self) -> (Key, Vec<u8>) {
        let (hash, bytes) = self.0.finish();
        (Key::from(hash), bytes)
    }
}

impl Write for Writer {
    fn write(&mut self, buf: &[u8]) -> Result<usize> {
        self.0.write(buf)
    }

    fn flush(&mut self) -> Result<()> {
        Ok(())
    }
}
