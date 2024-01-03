use pangalactic_b64 as b64;
use serde::{de::DeserializeOwned, Serialize};

pub fn serialize<T>(v: &T) -> anyhow::Result<String>
where
    T: Serialize,
{
    let bytes = crate::serialize(v)?;
    Ok(b64::encode(bytes))
}

pub fn deserialize<S, T>(s: S) -> anyhow::Result<T>
where
    S: AsRef<[u8]>,
    T: DeserializeOwned,
{
    let bytes = b64::decode(s)?;
    let v = crate::deserialize(bytes)?;
    Ok(v)
}
