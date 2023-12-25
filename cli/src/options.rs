use crate::dagops::{AnyPathDo, DagOps, LinkDo};
use clap::{Args, Parser, Subcommand};

#[derive(Debug, Parser)]
#[command(author, version, about, long_about = None)]
pub struct Options {
    #[command(subcommand)]
    command: Option<Command>,
}

impl Options {
    pub fn parse() -> Self {
        <Self as Parser>::parse()
    }

    pub async fn run(self) -> anyhow::Result<()> {
        use Command::*;
        use StoreCommand::*;

        match self.command.unwrap() {
            Store(cmd) => {
                let mut dops = DagOps::default();
                match cmd {
                    Put => dops.store_put().await,
                    Get(opts) => dops.store_get(&opts.link).await,
                    Copy(opts) => dops.store_copy(opts.source, opts.dest).await,
                }
            }
        }
    }
}

#[derive(Debug, Subcommand)]
pub enum Command {
    #[command(subcommand)]
    Store(StoreCommand),
}

/// Interact directly with the store
#[derive(Debug, Subcommand)]
pub enum StoreCommand {
    /// Insert the file on stdin and print its key on stdout
    Put,
    /// Send the given file to stdout
    Get(StoreGetOptions),
    /// Copy files or directories within or across store or host
    Copy(StoreCopyOptions),
}

#[derive(Debug, Args)]
pub struct StoreGetOptions {
    /// The link to get
    link: LinkDo,
}

#[derive(Debug, Args)]
pub struct StoreCopyOptions {
    /// The source path
    source: AnyPathDo,
    /// The destination path
    dest: AnyPathDo,
}
