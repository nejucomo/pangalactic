use pangalactic_store::Store;
use std::io::Result as IOResult;

#[derive(derive_more::From)]
pub struct CryptStore<S>(S);

impl<S> Store for CryptStore<S>
where
    S: Store,
{
    type Key = crate::ReadCap<<S as Store>::Key>;
    type Reader = std::io::Cursor<Vec<u8>>;
    type Writer = crate::Writer<<S as Store>::Writer>;

    fn open_writer(&self) -> IOResult<Self::Writer> {
        let w = self.0.open_writer()?;
        Ok(Self::Writer::new(w))
    }

    fn open_reader(&self, key: &Self::Key) -> IOResult<Self::Reader> {
        let plaintext = self.read(key)?;
        Ok(std::io::Cursor::new(plaintext))
    }

    fn read(&self, key: &Self::Key) -> IOResult<Vec<u8>> {
        let ciphertext = self.0.read(&key.basekey)?;
        let plaintext = key.sekey.unseal(&ciphertext).map_err(|()| {
            use std::io::{Error, ErrorKind};

            Error::new(
                ErrorKind::InvalidData,
                format!("Corrupted ciphertext: {:?}", ciphertext),
            )
        })?;
        Ok(plaintext)
    }
}
