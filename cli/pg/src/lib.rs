//! pangalactic cli library
//!
//! This crate also include the `pg` binary which is a thin wrapper around [run()].

#![feature(iter_intersperse)]
#![allow(async_fn_in_trait)]
pub mod options;
mod run;

pub use self::run::run;
