// TODO: This design doesn't address GC / data availability.

#![allow(async_fn_in_trait)]

mod commit;
mod load;
mod store;

pub use self::commit::Commit;
pub use self::load::Load;
pub use self::store::Store;
