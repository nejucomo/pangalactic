use serde::{Deserialize, Serialize};

pub fn serialize<T>(v: &T) -> anyhow::Result<Vec<u8>>
where
    T: Serialize,
{
    let buf = postcard::to_stdvec(v)?;
    Ok(buf)
}

pub fn deserialize<'a, T>(buf: &'a [u8]) -> anyhow::Result<T>
where
    T: Deserialize<'a>,
{
    let v = postcard::from_bytes(buf)?;
    Ok(v)
}

#[cfg(feature = "testutil")]
pub fn check_serialize_then_deserialize_equality<T>(input: T) -> anyhow::Result<()>
where
    T: std::cmp::PartialEq + std::fmt::Debug + serde::de::DeserializeOwned + Serialize,
{
    let buf = serialize(&input)?;
    let output = deserialize(&buf)?;
    assert_eq!(input, output);
    Ok(())
}
