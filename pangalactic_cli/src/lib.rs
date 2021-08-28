mod app;
pub mod cmd;
pub mod cmdexec;
pub mod opts;
pub mod store;

pub use app::app_main;

pub const APP_NAME: &'static str = "pg";
pub const PG_REPO_ATTIC: &'static str = ".pg";
