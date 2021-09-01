mod app;
pub mod cmd;
pub mod cmdexec;
pub mod display;
pub mod opts;
pub mod repo;
pub mod store;

pub use app::{app_main, get_appdirs};

pub const APP_NAME: &'static str = "pg";
pub const PG_REPO_CONTROL: &'static str = ".pg";
