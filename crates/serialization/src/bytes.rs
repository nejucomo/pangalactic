use serde::{de::DeserializeOwned, Serialize};

pub fn serialize<T>(v: &T) -> anyhow::Result<Vec<u8>>
where
    T: Serialize,
{
    let buf = postcard::to_stdvec(v)?;
    Ok(buf)
}

pub fn deserialize<B, T>(buf: B) -> anyhow::Result<T>
where
    B: AsRef<[u8]>,
    T: DeserializeOwned,
{
    let v = postcard::from_bytes(buf.as_ref())?;
    Ok(v)
}
