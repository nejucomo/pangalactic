use crate::Hash;

#[derive(Debug, Default)]
pub struct Hasher(blake3::Hasher);

impl Hasher {
    pub fn finalize(&self) -> Hash {
        Hash::wrap(self.0.finalize())
    }
}

impl std::io::Write for Hasher {
    fn write(&mut self, buf: &[u8]) -> std::io::Result<usize> {
        self.0.write(buf)
    }

    fn flush(&mut self) -> std::io::Result<()> {
        self.0.flush()
    }
}
