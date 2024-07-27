//! pangalactic cli library
//!
//! This crate also include the `pg` binary which is a thin wrapper around [run()].
pub mod cmd;
pub mod options;
mod run;
mod runnable;
pub mod store;

pub use self::run::run;
pub use self::runnable::Runnable;
