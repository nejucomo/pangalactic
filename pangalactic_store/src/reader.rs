use crate::key::Key;
use std::fs::File;
use std::io::Read;
use std::path::Path;

pub struct Reader {
    f: File,
    key: Key,
    hasher: blake3::Hasher,
}

impl Reader {
    pub fn open(dir: &Path, key: Key) -> std::io::Result<Reader> {
        let entrypath = dir.join(key.b64());
        let f = File::open(entrypath)?;
        let hasher = blake3::Hasher::new();
        Ok(Reader { f, key, hasher })
    }
}

impl Read for Reader {
    fn read(&mut self, buf: &mut [u8]) -> std::io::Result<usize> {
        let c = self.f.read(buf)?;
        self.hasher.update(&buf[..c]);
        Ok(c)
    }
}

impl crate::ReadVerify for Reader {
    fn verify(self) -> std::io::Result<()> {
        let actual = Key::from(self.hasher.finalize());
        if actual == self.key {
            Ok(())
        } else {
            use std::io::{Error, ErrorKind::InvalidData};
            let emsg = format!(
                "Data integrity mismatch for {:?}: calculated hash does not match {:?}",
                &self.key, actual,
            );
            Err(Error::new(InvalidData, emsg))
        }
    }
}
