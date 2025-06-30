use crate::{deserialize, serialize};

#[cfg(feature = "testutil")]
pub fn check_serialize_then_deserialize_equality<T>(input: T) -> anyhow::Result<()>
where
    T: std::cmp::PartialEq + std::fmt::Debug + serde::de::DeserializeOwned + serde::Serialize,
{
    let buf = serialize(dbg!(&input))?;
    let output = deserialize(dbg!(&buf))?;
    assert_eq!(input, dbg!(output));
    Ok(())
}
