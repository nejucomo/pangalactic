use anyhow::Result;
use async_trait::async_trait;
use clap::{Args, Parser, Subcommand};
use enum_dispatch::enum_dispatch;
use pangalactic_path::{AnyDestination, AnySource};
use pangalactic_stdstore::{
    StandardAnyDestination, StandardAnySource, StandardPath, StandardStore,
};

#[async_trait]
#[enum_dispatch]
pub trait Runnable {
    async fn run(self) -> Result<Option<StandardPath>>;
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

#[async_trait]
impl Runnable for Options {
    async fn run(self) -> Result<Option<StandardPath>> {
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

#[async_trait]
impl Runnable for StorePutOptions {
    async fn run(self) -> Result<Option<StandardPath>> {
        let mut store = StandardStore::default();
        store
            .transfer(AnySource::Stdin, AnyDestination::Store(None))
            .await
    }
}

/// Send the given file to stdout
#[derive(Debug, Args)]
pub struct StoreGetOptions {
    /// The source to get
    pub source: StandardAnySource,
}

#[async_trait]
impl Runnable for StoreGetOptions {
    async fn run(self) -> Result<Option<StandardPath>> {
        let mut store = StandardStore::default();
        store.transfer(self.source, AnyDestination::Stdout).await?;
        Ok(None)
    }
}

/// Transfer from SOURCE to DEST
#[derive(Debug, Args)]
pub struct StoreXferOptions {
    pub source: StandardAnySource,
    pub dest: StandardAnyDestination,
}

#[async_trait]
impl Runnable for StoreXferOptions {
    async fn run(self) -> Result<Option<StandardPath>> {
        let mut store = StandardStore::default();
        store.transfer(self.source, self.dest).await
    }
}
