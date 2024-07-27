mod dest;
mod source;

pub use self::dest::Destination;
pub use self::source::Source;

use crate::cmd::StoreCommander;
use crate::store::CliLink;
use clap::{Args, Parser, Subcommand};

#[derive(Debug, Parser)]
#[command(author, version, about, long_about = None)]
pub struct Options {
    #[command(subcommand)]
    pub command: Option<Command>,
}

impl Options {
    pub fn parse() -> Self {
        <Self as Parser>::parse()
    }

    pub async fn run(self) -> anyhow::Result<()> {
        use Command::*;

        match self.command.unwrap() {
            Store(cmd) => {
                use StoreCommand::*;

                let mut sc = StoreCommander::default();
                match cmd {
                    Put => {
                        let link = sc.put().await?;
                        println!("{link}");
                        Ok(())
                    }
                    Get(StoreGetOptions { link }) => sc.get(&link).await,
                    Xfer(XferOptions { source, dest }) => {
                        if let Some(link) = sc.xfer(&source, &dest).await? {
                            println!("{link}");
                        }
                        Ok(())
                    }
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
    Get(StoreGetOptions),
    Xfer(XferOptions),
}

/// Send the given file to stdout
#[derive(Debug, Args)]
pub struct StoreGetOptions {
    /// The link to get
    pub link: CliLink,
}

/// Transfer from SOURCE to DEST
#[derive(Debug, Args)]
pub struct XferOptions {
    pub source: Source,
    pub dest: Destination,
}
