use std::io::Result as IOResult;

pub use blake3::Hash;
pub use blake3::OUT_LEN as HASH_LENGTH;

pub struct HashSpool<W> {
    w: W,
    hasher: blake3::Hasher,
}

impl<W> HashSpool<W> {
    pub fn new(w: W) -> HashSpool<W> {
        let hasher = blake3::Hasher::new();
        HashSpool { w, hasher }
    }

    pub fn finish(self) -> (Hash, W) {
        let hash = self.hasher.finalize();
        (hash, self.w)
    }
}

impl<W> std::io::Write for HashSpool<W>
where
    W: std::io::Write,
{
    fn write(&mut self, buf: &[u8]) -> IOResult<usize> {
        self.w.write_all(buf)?;
        self.hasher.update(buf);
        Ok(buf.len())
    }

    fn flush(&mut self) -> IOResult<()> {
        self.w.flush()
    }
}
