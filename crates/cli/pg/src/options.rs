use clap::{Parser, Subcommand};
use pangalactic_cli_derive as pgderive;
use pangalactic_cli_revcon::options as revcon;
use pangalactic_cli_store::options as store;
use pangalactic_store_dirdb::DirDbStore;

#[derive(Debug, Parser)]
#[clap(author, version, about, long_about = None)]
pub struct PgOptions {
    /// The path to the dirdb store directory
    #[clap(short, long, default_value_t)]
    pub dirdb: DirDbStore,

    #[clap(subcommand)]
    pub command: Option<PgCommand>,
}

#[derive(Debug, Subcommand)]
pub enum PgCommand {
    #[clap(flatten)]
    RevCon(revcon::Command),
    #[clap(subcommand)]
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
    #[clap(subcommand, name = "revcon")]
    RevCon(revcon::Command),
    #[clap(subcommand)]
    Store(store::Command),
    Derive(pgderive::Options),
}
