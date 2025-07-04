use pangalactic_cid::ContentIdentifier;
use serde::{Deserialize, Serialize};
use std::fmt;
use std::str::FromStr;

pub type HashBytes = [u8; blake3::OUT_LEN];

#[derive(Clone, Debug, Eq, PartialEq, Hash, Deserialize, Serialize)]
#[serde(from = "HashBytes", into = "HashBytes")]
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

impl ContentIdentifier for Hash {}

impl From<HashBytes> for Hash {
    fn from(bytes: HashBytes) -> Self {
        Hash(blake3::Hash::from(bytes))
    }
}

impl From<Hash> for HashBytes {
    fn from(h: Hash) -> Self {
        h.0.into()
    }
}

impl FromStr for Hash {
    type Err = anyhow::Error;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        // TODO: perf: This copies twice. Can we do b64 -> [u8; K] in one pass?
        let bytes = pangalactic_b64::decode(s)?;
        let blen = bytes.len();
        let buf = <HashBytes>::try_from(bytes)
            .map_err(|_| anyhow::anyhow!("found {blen} bytes, expected {}", blake3::OUT_LEN))?;
        Ok(Hash(blake3::Hash::from(buf)))
    }
}

impl fmt::Display for Hash {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        pangalactic_b64::encode(self.0.as_bytes()).fmt(f)
    }
}
