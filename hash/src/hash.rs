use pangalactic_store::StoreCid;
use serde::{Deserialize, Serialize};
use std::fmt;
use std::str::FromStr;

#[derive(Clone, Debug, Eq, PartialEq, Hash, Deserialize, Serialize)]
#[serde(from = "[u8; blake3::OUT_LEN]", into = "[u8; blake3::OUT_LEN]")]
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

impl StoreCid for Hash {
    fn encode_fields(&self, dest: &mut Vec<String>) {
        pangalactic_store::cid_encode_fields_from_display(self, dest);
    }

    fn parse_fields<'a, I>(fields: I) -> anyhow::Result<Self>
    where
        I: Iterator<Item = &'a str>,
    {
        pangalactic_store::cid_decode_fields_fromstr(fields)
    }
}

impl From<[u8; blake3::OUT_LEN]> for Hash {
    fn from(bytes: [u8; blake3::OUT_LEN]) -> Self {
        Hash(blake3::Hash::from(bytes))
    }
}

impl From<Hash> for [u8; blake3::OUT_LEN] {
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
