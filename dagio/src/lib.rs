// Documentation readability hack; see https://github.com/dtolnay/async-trait/issues/213#issuecomment-1559690487
#![cfg_attr(doc, feature(async_fn_in_trait))]

mod aliases;
mod commit;
mod dagio;
mod dirimpl;
mod load;
mod reader;
mod writer;

pub use self::aliases::{DagioHostDirectory, DagioLink};
pub use self::commit::DagioCommit;
pub use self::dagio::Dagio;
pub use self::load::DagioLoad;
pub use self::reader::DagioReader;
pub use self::writer::DagioWriter;

#[cfg(test)]
mod tests;
