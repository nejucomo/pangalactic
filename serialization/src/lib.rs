// Documentation readability hack; see https://github.com/dtolnay/async-trait/issues/213#issuecomment-1559690487
#![cfg_attr(doc, feature(async_fn_in_trait))]

mod deserialize;
mod serialize;

pub mod flexint;
pub mod testutil;

pub use self::deserialize::AsyncDeserialize;
pub use self::serialize::AsyncSerialize;

#[cfg(test)]
mod tests;
