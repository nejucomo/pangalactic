use serde::de::DeserializeOwned;
use serde::Serialize;
use std::fmt::{Debug, Display};
use std::str::FromStr;

/// `CID` is an acronym for `Content IDentifier` required to have these properties beyond the type signature:
///
/// - Inserting the same bytes sequence into a store multiple times produces the same `CID` on any host.
/// - Two distinct byte sequences never produce the same `CID` upon insertion into the store on any host.
/// - A `CID` should be concise.
///
/// Cryptographic hash functions over the content are assumed to meet these properties.
pub trait StoreCid: Clone + Eq + Debug + Serialize + DeserializeOwned + Send + Sync {
    fn encode_fields(&self, dest: &mut Vec<String>);

    fn parse_fields<'a, I>(fields: I) -> anyhow::Result<Self>
    where
        I: Iterator<Item = &'a str>;
}

pub fn cid_encode_fields_from_display<T>(v: &T, dest: &mut Vec<String>)
where
    T: Display,
{
    dest.push(v.to_string())
}

pub fn cid_decode_fields_fromstr<'a, T, I>(mut fields: I) -> anyhow::Result<T>
where
    T: FromStr<Err = anyhow::Error>,
    I: Iterator<Item = &'a str>,
{
    let field = fields
        .next()
        .ok_or_else(|| anyhow::anyhow!("missing {} field", std::any::type_name::<T>()))?;

    if let Some(f) = fields.next() {
        anyhow::bail!("unexpected field: {f:?}");
    } else {
        field.parse()
    }
}
