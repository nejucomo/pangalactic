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
pub trait StoreCid:
    Clone
    + Eq
    + Debug
    + Display
    + FromStr<Err = anyhow::Error>
    + Serialize
    + DeserializeOwned
    + Send
    + Sync
{
}
