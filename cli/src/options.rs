mod dest;
mod source;

pub use self::dest::Destination;
pub use self::source::Source;

use async_trait::async_trait;
use clap::{Args, Parser, Subcommand};
use enum_dispatch::enum_dispatch;
use pangalactic_layer_cidmeta::CidMeta;
use pangalactic_link::Link;
use pangalactic_store::Store;
use pangalactic_store_dirdb::DirDbStore;

use crate::cmd::StoreCommander;

#[cfg_attr(not(doc), async_trait)]
#[enum_dispatch]
pub trait Runnable {
    async fn run(self) -> anyhow::Result<()>;
}

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
}

#[cfg_attr(not(doc), async_trait)]
impl Runnable for Options {
    async fn run(self) -> anyhow::Result<()> {
        self.command.unwrap().run().await
    }
}

#[enum_dispatch(Runnable)]
#[derive(Debug, Subcommand)]
pub enum Command {
    #[command(subcommand)]
    Store(StoreCommand),
}

/// Interact directly with the store
#[enum_dispatch(Runnable)]
#[derive(Debug, Subcommand)]
pub enum StoreCommand {
    Put(StorePutOptions),
    Get(StoreGetOptions),
    Xfer(StoreXferOptions),
}

/// Insert the file on stdin and print its key on stdout
#[derive(Debug, Args)]
pub struct StorePutOptions {}

#[cfg_attr(not(doc), async_trait)]
impl Runnable for StorePutOptions {
    async fn run(self) -> anyhow::Result<()> {
        let mut sc = StoreCommander::default();
        let link = sc.put().await?;
        println!("{link}");
        Ok(())
    }
}

/// Send the given file to stdout
#[derive(Debug, Args)]
pub struct StoreGetOptions {
    /// The link to get
    pub link: Link<CidMeta<<DirDbStore as Store>::CID>>,
}

#[cfg_attr(not(doc), async_trait)]
impl Runnable for StoreGetOptions {
    async fn run(self) -> anyhow::Result<()> {
        let mut sc = StoreCommander::default();
        sc.get(&self.link).await
    }
}

/// Transfer from SOURCE to DEST
#[derive(Debug, Args)]
pub struct StoreXferOptions {
    pub source: Source,
    pub dest: Destination,
}

#[cfg_attr(not(doc), async_trait)]
impl Runnable for StoreXferOptions {
    async fn run(self) -> anyhow::Result<()> {
        let mut sc = StoreCommander::default();
        if let Some(link) = sc.xfer(&self.source, &self.dest).await? {
            println!("{link}");
        }
        Ok(())
    }
}
