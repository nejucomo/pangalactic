use clap::{Args, Parser, Subcommand};

/// Manage the pg seed directory
#[derive(Debug, Parser)]
#[command(author, version, about, long_about = None)]
pub struct Options {
    #[command(subcommand)]
    pub command: Command,
}

/// Manage the pg seed directory
#[derive(Debug, Subcommand)]
pub enum Command {
    List(ListOptions),
    Install(InstallOptions),
}

/// List the pgwasm names
#[derive(Debug, Args)]
pub struct ListOptions {}

/// Install the stdlib pgwasm directory
#[derive(Debug, Args)]
pub struct InstallOptions {}
