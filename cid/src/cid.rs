use std::fmt::Debug;

use serde::de::DeserializeOwned;
use serde::Serialize;

/// A `ContentIdentifier` is required to have these properties beyond the type signature:
///
/// - Inserting the same bytes sequence into a store multiple times produces the same value on any host.
/// - Two distinct byte sequences never produce the same value upon insertion into the store on any host.
/// - It should be concise.
///
/// Cryptographic hash functions over the content are assumed to meet these properties.
pub trait ContentIdentifier:
    Clone + Eq + Debug + Serialize + DeserializeOwned + Send + Sync
{
}
