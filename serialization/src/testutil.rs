use crate::{AsyncDeserialize, AsyncSerialize};

pub async fn check_serialize_then_deserialize_equality<T>(input: T)
where
    T: AsyncSerialize + AsyncDeserialize + PartialEq + std::fmt::Debug,
{
    let mut buf = vec![];

    dbg!(&input);
    input.write_into(&mut buf).await.unwrap();
    dbg!(&buf);
    let output = T::read_from(buf.as_slice()).await.unwrap();

    assert_eq!(input, output);
}
