use async_trait::async_trait;
use pangalactic_serialization::{AsyncDeserialize, AsyncSerialize};
use std::fmt;
use std::marker::Unpin;
use std::str::FromStr;
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

impl FromStr for Hash {
    type Err = anyhow::Error;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        // TODO: perf: This copies twice. Can we do b64 -> [u8; K] in one pass?
        let bytes = pangalactic_b64::decode(s)?;
        let blen = bytes.len();
        let buf = <[u8; blake3::OUT_LEN]>::try_from(bytes)
            .map_err(|_| anyhow::anyhow!("found {blen} bytes, expected {}", blake3::OUT_LEN))?;
        Ok(Hash(blake3::Hash::from(buf)))
    }
}

impl fmt::Display for Hash {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        pangalactic_b64::encode(self.0.as_bytes()).fmt(f)
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
