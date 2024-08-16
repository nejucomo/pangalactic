use std::{future::Future, pin::Pin};

use anyhow::Result;
use clap::{Args, Parser, Subcommand};
use enum_dispatch::enum_dispatch;
use pangalactic_layer_dir::LinkDirectoryStore;
use pangalactic_layer_host::{
    HostAnyDestination, HostAnySource, HostLayer, HostLinkDirectory, HostStorePath,
};
use pangalactic_layer_path::{AnyDestination, AnySource};
use pangalactic_store::Store;
use pangalactic_store_dirdb::DirDbStore;

type CliAnyDestination = HostAnyDestination<DirDbStore>;
type CliAnySource = HostAnySource<DirDbStore>;
type CliStore = HostLayer<DirDbStore>;
type CliStorePath = HostStorePath<DirDbStore>;
type CliLinkDirectory = HostLinkDirectory<DirDbStore>;

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
    /// The plan to derive, or an exec file if `INPUT` is provided
    pub plan_or_exec: CliAnySource,

    /// An input to derive; if absent `PLAN_OR_EXEC` must be a plan
    pub input: Option<CliAnySource>,
}

impl Runnable for DeriveOptions {
    fn run(self) -> Pin<Box<dyn Future<Output = Result<Option<CliStorePath>>>>> {
        use pangalactic_schemata::Plan;

        Box::pin(async {
            let mut store = CliStore::default();

            // Transfer any source into the store to get a store path:
            // Assert: Final unwrap never fails because `AnyDestination::Store` always produces a path:
            let plan_or_exec = store
                .transfer(self.plan_or_exec, AnyDestination::Store(None))
                .await?
                .unwrap();

            let plan = if let Some(input) = self.input {
                let input_path = store
                    .transfer(input, AnyDestination::Store(None))
                    .await?
                    .unwrap();
                let exec = store.resolve_path(&plan_or_exec).await?;
                let input = store.resolve_path(&input_path).await?;
                store
                    .commit(Plan { exec, input })
                    .await
                    .map(CliStorePath::from)?
            } else {
                plan_or_exec
            };

            let attestation = store.derive(plan).await?;
            Ok(Some(attestation))
        })
    }
}

/// Manage the stdlib of pgwasm guests
#[enum_dispatch(Runnable)]
#[derive(Debug, Subcommand)]
pub enum StdlibCommand {
    List(StdlibListOptions),
    Install(StdlibInstallOptions),
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

/// Install the stdlib pgwasm directory
#[derive(Debug, Args)]
pub struct StdlibInstallOptions {}

impl Runnable for StdlibInstallOptions {
    fn run(self) -> Pin<Box<dyn Future<Output = Result<Option<CliStorePath>>>>> {
        Box::pin(async {
            let mut store = CliStore::default();

            let mut linkdir = CliLinkDirectory::default();
            for name in pangalactic_guests::iter_wasm_names() {
                let bytes = pangalactic_guests::get_wasm_bytes(name)?;
                let link = store.commit_to_link(bytes).await?;
                let fname = format!("{name}.wasm");
                tracing::debug!(?fname, ?link, "committed wasm");
                linkdir.insert(fname, link)?;
            }
            let link = store.commit(linkdir).await?;

            Ok(Some(CliStorePath::from(link)))
        })
    }
}
