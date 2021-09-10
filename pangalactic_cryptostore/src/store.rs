use pangalactic_store::{KeyOf, Store, WriterOf};
use std::io::Result as IOResult;

#[derive(Debug, derive_more::From)]
pub struct CryptoStore<S>(S);

impl<S> Store for CryptoStore<S>
where
    S: Store,
{
    type Key = crate::ReadCap<KeyOf<S>>;
    type Reader = std::io::Cursor<Vec<u8>>;
    type Writer = crate::Writer<WriterOf<S>>;

    fn open_reader(&self, key: &Self::Key) -> IOResult<Self::Reader> {
        let plaintext = self.read_bytes(key)?;
        Ok(std::io::Cursor::new(plaintext))
    }

    fn read_bytes(&self, key: &Self::Key) -> IOResult<Vec<u8>> {
        let ciphertext = self.0.read_bytes(&key.basekey)?;
        let plaintext = key.sekey.unseal(&ciphertext).map_err(|()| {
            use std::io::{Error, ErrorKind};

            Error::new(
                ErrorKind::InvalidData,
                format!("Corrupted ciphertext: {:?}", ciphertext),
            )
        })?;
        Ok(plaintext)
    }

    fn open_writer(&self) -> IOResult<Self::Writer> {
        let w = self.0.open_writer()?;
        Ok(Self::Writer::new(w))
    }

    fn commit_writer(&mut self, w: Self::Writer) -> IOResult<Self::Key> {
        use std::io::Write;

        let (mut inner, sekey, ciphertext) = w.finish();
        inner.write_all(&ciphertext[..])?;
        let basekey = self.0.commit_writer(inner)?;
        Ok(crate::ReadCap { basekey, sekey })
    }
}
