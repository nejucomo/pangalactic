// TODO: This design doesn't address GC / data availability.

// Documentation readability hack; see https://github.com/dtolnay/async-trait/issues/213#issuecomment-1559690487
#![cfg_attr(doc, feature(async_fn_in_trait))]

mod commit;
mod load;
mod store;

pub use self::commit::Commit;
pub use self::load::Load;
pub use self::store::Store;
