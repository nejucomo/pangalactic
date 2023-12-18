//! pangalactic cli library
//!
//! This crate also include the `pg` binary which is a thin wrapper around [run()].
pub mod cmd;
pub mod options;
mod run;

pub use self::run::run;
