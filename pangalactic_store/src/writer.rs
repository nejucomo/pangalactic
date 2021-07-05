use crate::randtoken;
use std::fs::File;
use std::io::Write;
use std::path::{Path, PathBuf};

pub struct Writer {
    dir: PathBuf,
    spoolpath: PathBuf,
    spool: File,
    hasher: blake3::Hasher,
}

impl Writer {
    pub fn open(dir: &Path) -> std::io::Result<Writer> {
        let spoolpath = dir.join(format!("in.{}", randtoken::generate()));
        let spool = File::create(&spoolpath)?;
        let hasher = blake3::Hasher::new();
        Ok(Writer {
            dir: PathBuf::from(dir),
            spoolpath,
            spool,
            hasher,
        })
    }
}

impl Write for Writer {
    fn write(&mut self, buf: &[u8]) -> std::io::Result<usize> {
        self.spool.write_all(buf)?;
        self.hasher.update(buf);
        Ok(buf.len())
    }

    fn flush(&mut self) -> std::io::Result<()> {
        self.spool.flush()
    }
}

impl crate::WriteCommit for Writer {
    type Key = crate::key::Key;

    fn commit(self) -> std::io::Result<Self::Key> {
        // Induce a file closure.
        // TODO: Verify this induces file to close:
        std::mem::drop(self.spool);

        let key = Self::Key::from(self.hasher.finalize());
        let entrypath = self.dir.join(&key.b64());

        // BUG: The semantics we want for all platforms are that if the destination does not exist,
        // the operation succeeds; if the destination does exist, the operation fails in a specific
        // way, and any other errors are propagated. Furthermore, moves need to be atomic, such
        // that any pre-existing destination is never modified or else the newly created
        // destination exactly matches our spool contents. These semantics guarantee if multiple
        // processes attempt to commit the same contents, the end result for both processes is
        // safely that the new entry is successfully created, regardless of which process creates
        // the entry. It sounds like `std::fs::rename` may not guarantee these semantics and also
        // it's functionality may change in the future.
        // TODO: detect the case that the entry already (correctly) exists, and return Ok.
        std::fs::rename(self.spoolpath, entrypath)?;

        Ok(key)
    }
}
