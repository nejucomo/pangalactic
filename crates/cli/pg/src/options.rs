use clap::{Parser, Subcommand};
use pangalactic_cli_derive::options as derive;
use pangalactic_cli_revcon::options as revcon;
use pangalactic_cli_store::options as store;

#[derive(Debug, Parser)]
#[command(author, version, about, long_about = None)]
pub struct PgOptions {
    #[command(subcommand)]
    pub command: Option<PgCommand>,
}

#[derive(Debug, Subcommand)]
pub enum PgCommand {
    #[command(flatten)]
    RevCon(revcon::Command),
    #[command(subcommand)]
    Util(UtilCommand),
}

impl Default for PgCommand {
    fn default() -> Self {
        PgCommand::RevCon(revcon::Command::default())
    }
}

/// General Utilities
#[derive(Debug, Subcommand)]
pub enum UtilCommand {
    #[command(subcommand, name = "revcon")]
    RevCon(revcon::Command),
    #[command(subcommand)]
    Store(store::Command),
    Derive(derive::Options),
}
