use crate::sekbox::SEKey;
use pangalactic_hashspool::{HashSpool, HASH_LENGTH};
use std::io::Result as IOResult;

pub struct Writer<W> {
    inner: W,
    hashspool: HashSpool<Vec<u8>>,
}

impl<W> Writer<W> {
    pub(crate) fn new(inner: W) -> Writer<W> {
        let hashspool = HashSpool::new(vec![]);
        Writer { inner, hashspool }
    }

    pub(crate) fn finish(self) -> (W, SEKey, Vec<u8>) {
        use crate::sekbox::KEY_LENGTH;
        use static_assertions::const_assert_eq;

        const_assert_eq!(HASH_LENGTH, KEY_LENGTH);

        let (hash, cleartext) = self.hashspool.finish();
        // unwrap guaranteed by const_assert_eq:
        let sekey = SEKey::from(hash.as_bytes());
        let ciphertext = sekey.seal(&cleartext[..]);
        (self.inner, sekey, ciphertext)
    }
}

impl<W> std::io::Write for Writer<W>
where
    W: std::io::Write,
{
    fn write(&mut self, buf: &[u8]) -> IOResult<usize> {
        self.hashspool.write(buf)
    }

    fn flush(&mut self) -> IOResult<()> {
        Ok(())
    }
}
