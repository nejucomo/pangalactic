use crate::ReadCap;
use pangalactic_hashspool::{HashSpool, HASH_LENGTH};
use pangalactic_store::WriteCommit;
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

impl<W> WriteCommit for Writer<W>
where
    W: WriteCommit,
{
    type Key = ReadCap<<W as WriteCommit>::Key>;

    fn commit(self) -> IOResult<Self::Key> {
        use crate::sekbox::{SEKey, KEY_LENGTH};
        use static_assertions::const_assert_eq;

        const_assert_eq!(HASH_LENGTH, KEY_LENGTH);

        let (hash, cleartext) = self.hashspool.finish();
        // unwrap guaranteed by const_assert_eq:
        let sekey = SEKey::from(hash.as_bytes());
        let ciphertext = sekey.seal(&cleartext[..]);
        let basekey = self.inner.write_all_and_commit(&ciphertext[..])?;
        Ok(ReadCap { basekey, sekey })
    }
}
