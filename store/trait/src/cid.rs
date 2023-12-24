use serde::de::DeserializeOwned;
use serde::Serialize;
use std::fmt::Debug;

/// A `StoreCid` uniquely identifies immutable content
///
/// `Cid` is an acronym for `Content IDentifier` required to have these properties beyond the type
/// signature:
///
/// - Inserting the same bytes sequence into a store multiple times produces the same `Cid` on
///   any host.
/// - Two distinct byte sequences never produce the same `Cid` upon insertion into the store on
///   any host.
/// - A `Cid` should be concise.
///
/// Cryptographic hash functions over the content are assumed to meet these properties.
pub trait StoreCid: Clone + Eq + Debug + Serialize + DeserializeOwned + Send + Sync {
    /// The URL scheme for links using this Cid
    const SCHEME: &'static str;
}
