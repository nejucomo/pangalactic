use serde::{de::DeserializeOwned, Serialize};

pub fn serialize<T>(v: &T) -> anyhow::Result<Vec<u8>>
where
    T: Serialize,
{
    let buf = postcard::to_stdvec(v)?;
    Ok(buf)
}

/// Deserialize bytes from the pangalactic binary format to `T`
///
/// # TODO
///
/// - Require strict versioning. *Correctness / Future Proofing*
/// - Change from [DeserializeOwned] to [serde::Deserialize] to reduce buffer copies. `buf` needs to have an explicit lifetime passed to [serde]. (Example `../../pubsub/src/envelope.rs`.) *Performance / API Ergonomics*
pub fn deserialize<B, T>(buf: B) -> anyhow::Result<T>
where
    B: AsRef<[u8]>,
    T: DeserializeOwned,
{
    let v = postcard::from_bytes(buf.as_ref())?;
    Ok(v)
}
