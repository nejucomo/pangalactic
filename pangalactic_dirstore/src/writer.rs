use crate::randtoken;
use pangalactic_hashspool::{Hash, HashSpool};
use std::fs::File;
use std::io::Result as IOResult;
use std::io::Write;
use std::path::{Path, PathBuf};

pub struct Writer {
    dir: PathBuf,
    spoolpath: PathBuf,
    hashspool: HashSpool<File>,
}

impl Writer {
    pub(crate) fn open(dir: &Path) -> IOResult<Writer> {
        let dir = PathBuf::from(dir);
        let spoolpath = dir.join(format!("in.{}", randtoken::generate()));
        let hashspool = HashSpool::new(File::create(&spoolpath)?);
        Ok(Writer {
            dir,
            spoolpath,
            hashspool,
        })
    }

    pub(crate) fn commit(self) -> IOResult<Hash> {
        use pangalactic_codec as codec;

        let (key, f) = self.hashspool.finish();

        // Induce a file closure.
        // TODO: Verify this induces file to close:
        std::mem::drop(f);

        let entrypath = self.dir.join(codec::encode_string(&key));

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

impl Write for Writer {
    fn write(&mut self, buf: &[u8]) -> IOResult<usize> {
        self.hashspool.write_all(buf)?;
        Ok(buf.len())
    }

    fn flush(&mut self) -> IOResult<()> {
        self.hashspool.flush()
    }
}
