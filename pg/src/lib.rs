mod app;
pub mod cmd;
pub mod opts;
pub mod repo;

pub use app::get_appdirs;

pub const APP_NAME: &'static str = "pg";
pub const PG_REPO_CONTROL: &'static str = ".pg";
