use std::io::{Cursor, Write};
use std::path::Path;

pub struct Writer<'a> {
    dir: &'a Path,
    cursor: Cursor<Vec<u8>>,
    hasher: blake3::Hasher,
}

impl<'a> Writer<'a> {
    pub fn new(dir: &'a Path) -> Writer<'a> {
        let cursor = Cursor::new(vec![]);
        let hasher = blake3::Hasher::new();
        Writer {
            dir,
            cursor,
            hasher,
        }
    }

    pub fn commit(self) -> std::io::Result<String> {
        let hash = self.hasher.finalize();
        let hashkey = base64::encode_config(hash.as_bytes(), base64::URL_SAFE_NO_PAD);

        let mut f = std::fs::File::create(self.dir.join(&hashkey))?;
        f.write_all(&self.cursor.into_inner()[..])?;
        f.flush()?;
        Ok(hashkey)
    }
}

impl<'a> Write for Writer<'a> {
    fn write(&mut self, buf: &[u8]) -> std::io::Result<usize> {
        self.cursor.write_all(buf)?;
        self.hasher.update(buf);
        Ok(buf.len())
    }

    fn flush(&mut self) -> std::io::Result<()> {
        self.cursor.flush()
    }
}
