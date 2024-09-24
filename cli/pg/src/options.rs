use clap::{Parser, Subcommand};
use pangalactic_cli_derive::options::DeriveOptions;
use pangalactic_cli_revcon::options::RevConCommand;
use pangalactic_cli_store::options::StoreCommand;

#[derive(Debug, Parser)]
#[command(author, version, about, long_about = None)]
pub struct PgOptions {
    #[command(subcommand)]
    pub command: Option<PgCommand>,
}

#[derive(Debug, Subcommand)]
pub enum PgCommand {
    #[command(flatten)]
    RevCon(RevConCommand),
    #[command(subcommand)]
    Util(UtilCommand),
}

impl Default for PgCommand {
    fn default() -> Self {
        PgCommand::RevCon(RevConCommand::default())
    }
}

/// General Utilities
#[derive(Debug, Subcommand)]
pub enum UtilCommand {
    #[command(subcommand, name = "revcon")]
    RevCon(RevConCommand),
    #[command(subcommand)]
    Store(StoreCommand),
    Derive(DeriveOptions),
}
