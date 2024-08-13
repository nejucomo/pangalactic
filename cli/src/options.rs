use std::{future::Future, pin::Pin};

use anyhow::Result;
use clap::{Args, Parser, Subcommand};
use enum_dispatch::enum_dispatch;
use pangalactic_layer_host::{HostAnyDestination, HostAnySource, HostLayer, HostStorePath};
use pangalactic_layer_path::{AnyDestination, AnySource};
use pangalactic_store_dirdb::DirDbStore;

type CliAnyDestination = HostAnyDestination<DirDbStore>;
type CliAnySource = HostAnySource<DirDbStore>;
type CliStore = HostLayer<DirDbStore>;
type CliStorePath = HostStorePath<DirDbStore>;

// Upstream Bug: `enum_dispatch` does not support `async fn` in traits. :-(
#[enum_dispatch]
pub trait Runnable {
    fn run(self) -> Pin<Box<dyn Future<Output = Result<Option<CliStorePath>>>>>;
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
    fn run(self) -> Pin<Box<dyn Future<Output = Result<Option<CliStorePath>>>>> {
        self.command.unwrap().run()
    }
}

#[enum_dispatch(Runnable)]
#[derive(Debug, Subcommand)]
pub enum Command {
    #[command(subcommand)]
    Store(StoreCommand),
    Derive(DeriveOptions),
    #[command(subcommand)]
    Stdlib(StdlibCommand),
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
    fn run(self) -> Pin<Box<dyn Future<Output = Result<Option<CliStorePath>>>>> {
        Box::pin(async {
            let mut store = CliStore::default();
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
    pub source: CliAnySource,
}

impl Runnable for StoreGetOptions {
    fn run(self) -> Pin<Box<dyn Future<Output = Result<Option<CliStorePath>>>>> {
        Box::pin(async {
            let mut store = CliStore::default();
            store.transfer(self.source, AnyDestination::Stdout).await?;
            Ok(None)
        })
    }
}

/// Transfer from SOURCE to DEST
#[derive(Debug, Args)]
pub struct StoreXferOptions {
    pub source: CliAnySource,
    pub dest: CliAnyDestination,
}

impl Runnable for StoreXferOptions {
    fn run(self) -> Pin<Box<dyn Future<Output = Result<Option<CliStorePath>>>>> {
        Box::pin(async {
            let mut store = CliStore::default();
            store.transfer(self.source, self.dest).await
        })
    }
}

/// Derive a plan
#[derive(Debug, Args)]
pub struct DeriveOptions {
    /// The plan to derive
    pub plan: CliAnySource,
}

impl Runnable for DeriveOptions {
    fn run(self) -> Pin<Box<dyn Future<Output = Result<Option<CliStorePath>>>>> {
        Box::pin(async {
            let mut store = CliStore::default();
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

/// Manage the stdlib of pgwasm guests
#[enum_dispatch(Runnable)]
#[derive(Debug, Subcommand)]
pub enum StdlibCommand {
    List(StdlibListOptions),
}

/// List the pgwasm names
#[derive(Debug, Args)]
pub struct StdlibListOptions {}

impl Runnable for StdlibListOptions {
    fn run(self) -> Pin<Box<dyn Future<Output = Result<Option<CliStorePath>>>>> {
        Box::pin(async {
            for name in pangalactic_guests::iter_wasm_names() {
                println!("{name}");
            }
            Ok(None)
        })
    }
}
