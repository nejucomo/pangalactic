use std::io::{Result, Write};

#[derive(derive_more::From)]
pub struct FileWriter<W>(W);

impl<W> FileWriter<W> {
    pub(crate) fn unwrap(self) -> W {
        self.0
    }
}

impl<W> Write for FileWriter<W>
where
    W: Write,
{
    fn write(&mut self, buf: &[u8]) -> Result<usize> {
        self.0.write(buf)
    }

    fn flush(&mut self) -> Result<()> {
        self.0.flush()
    }
}
