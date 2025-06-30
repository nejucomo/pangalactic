//! Consistent configuration and data-dir management across modules
#![deny(unsafe_code, missing_docs)]
#![allow(async_fn_in_trait)]

mod config;
pub mod datapath;
mod pgdirs;

pub use self::config::Configuration;
