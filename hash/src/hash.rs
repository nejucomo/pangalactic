use async_trait::async_trait;
use pangalactic_serialization::{AsyncDeserialize, AsyncSerialize};
use std::marker::Unpin;
use tokio::io::{AsyncRead, AsyncWrite};

#[derive(Clone, Debug, Eq, PartialEq, Hash)]
pub struct Hash(blake3::Hash);

impl Hash {
    pub fn of<T>(t: T) -> Hash
    where
        T: AsRef<[u8]>,
    {
        use std::io::Write;

        let mut hasher = crate::Hasher::default();
        hasher.write_all(t.as_ref()).unwrap();
        hasher.finalize()
    }

    pub(crate) fn wrap(b3h: blake3::Hash) -> Self {
        Hash(b3h)
    }
}

#[async_trait]
impl AsyncSerialize for Hash {
    async fn write_into<W>(&self, w: W) -> anyhow::Result<()>
    where
        W: AsyncWrite + Unpin + Send,
    {
        self.0.as_bytes().write_into(w).await
    }
}

#[async_trait]
impl AsyncDeserialize for Hash {
    async fn read_from<R>(r: R) -> anyhow::Result<Self>
    where
        R: AsyncRead + Unpin + Send,
    {
        let buf = <[u8; blake3::OUT_LEN]>::read_from(r).await?;
        Ok(Hash(blake3::Hash::from(buf)))
    }
}
