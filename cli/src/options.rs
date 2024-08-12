use std::{future::Future, pin::Pin};

use anyhow::Result;
use clap::{Args, Parser, Subcommand};
use enum_dispatch::enum_dispatch;
use pangalactic_layer_path::{AnyDestination, AnySource};
use pangalactic_stdstore::{
    StandardAnyDestination, StandardAnySource, StandardPath, StandardStore,
};

// Upstream Bug: `enum_dispatch` does not support `async fn` in traits. :-(
#[enum_dispatch]
pub trait Runnable {
    fn run(self) -> Pin<Box<dyn Future<Output = Result<Option<StandardPath>>>>>;
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

impl Runnable for Options {
    fn run(self) -> Pin<Box<dyn Future<Output = Result<Option<StandardPath>>>>> {
        self.command.unwrap().run()
    }
}

#[enum_dispatch(Runnable)]
#[derive(Debug, Subcommand)]
pub enum Command {
    #[command(subcommand)]
    Store(StoreCommand),
    Derive(DeriveOptions),
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

impl Runnable for StorePutOptions {
    fn run(self) -> Pin<Box<dyn Future<Output = Result<Option<StandardPath>>>>> {
        Box::pin(async {
            let mut store = StandardStore::default();
            store
                .transfer(AnySource::Stdin, AnyDestination::Store(None))
                .await
        })
    }
}

/// Send the given file to stdout
#[derive(Debug, Args)]
pub struct StoreGetOptions {
    /// The source to get
    pub source: StandardAnySource,
}

impl Runnable for StoreGetOptions {
    fn run(self) -> Pin<Box<dyn Future<Output = Result<Option<StandardPath>>>>> {
        Box::pin(async {
            let mut store = StandardStore::default();
            store.transfer(self.source, AnyDestination::Stdout).await?;
            Ok(None)
        })
    }
}

/// Transfer from SOURCE to DEST
#[derive(Debug, Args)]
pub struct StoreXferOptions {
    pub source: StandardAnySource,
    pub dest: StandardAnyDestination,
}

impl Runnable for StoreXferOptions {
    fn run(self) -> Pin<Box<dyn Future<Output = Result<Option<StandardPath>>>>> {
        Box::pin(async {
            let mut store = StandardStore::default();
            store.transfer(self.source, self.dest).await
        })
    }
}

/// Derive a plan
#[derive(Debug, Args)]
pub struct DeriveOptions {
    /// The plan to derive
    pub plan: StandardAnySource,
}

impl Runnable for DeriveOptions {
    fn run(self) -> Pin<Box<dyn Future<Output = Result<Option<StandardPath>>>>> {
        Box::pin(async {
            let mut store = StandardStore::default();
            // Transfer any source into the store to get a store path:
            // Assert: Final unwrap never fails because `AnyDestination::Store` always produces a path:
            let plan = store
                .transfer(self.plan, AnyDestination::Store(None))
                .await?
                .unwrap();

            let attestation = store.derive(plan).await?;
            tracing::info!("{attestation}");
            Ok(None)
        })
    }
}
