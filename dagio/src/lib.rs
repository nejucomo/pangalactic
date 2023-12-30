// Documentation readability hack; see https://github.com/dtolnay/async-trait/issues/213#issuecomment-1559690487
#![cfg_attr(doc, feature(async_fn_in_trait))]

mod aliases;
mod dagio;
mod dirimpl;
mod fromdag;
mod todag;

pub use self::aliases::{DagioLink, HostDirectoryFor, WriterFor};
pub use self::dagio::Dagio;
pub use self::fromdag::DagioLoad;
pub use self::todag::DagioCommit;

#[cfg(test)]
mod tests;
