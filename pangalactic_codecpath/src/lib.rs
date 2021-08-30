use pangalactic_codec::DecodeBytesError;
use pangalactic_errorutil::into_std_error;
use serde::{de::DeserializeOwned, Serialize};
use std::path::{Path, PathBuf};

#[derive(Debug, derive_more::From)]
pub enum DecodeError {
    IO(std::io::Error),
    Codec(DecodeBytesError),
}

into_std_error!(DecodeError, std::io::ErrorKind::InvalidData);

pub trait CodecPath: AsRef<Path> {
    fn read_bytes(&self) -> std::io::Result<Vec<u8>> {
        use pangalactic_fs::file_open;
        use std::io::Read;

        let p = self.as_ref();
        let mut f = file_open(p)?;
        let mut bytes = vec![];
        f.read_to_end(&mut bytes)?;
        Ok(bytes)
    }

    fn write_bytes<T: AsRef<[u8]>>(&self, buf: T) -> std::io::Result<()> {
        use pangalactic_fs::file_create;
        use std::io::Write;

        let p = self.as_ref();
        let mut f = file_create(p)?;
        f.write_all(buf.as_ref())?;
        Ok(())
    }

    fn decode_contents<T: DeserializeOwned>(&self) -> Result<T, DecodeError> {
        use pangalactic_codec::decode_bytes;

        let bytes = self.read_bytes()?;
        let x: T = decode_bytes(&bytes[..])?;
        Ok(x)
    }

    fn create_with<T: Serialize>(&self, x: &T) -> std::io::Result<()> {
        use pangalactic_codec::encode_bytes;

        self.write_bytes(encode_bytes(x))
    }
}

impl CodecPath for Path {}
impl CodecPath for PathBuf {}
