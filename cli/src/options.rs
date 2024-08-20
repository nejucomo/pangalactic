use std::{fmt::Display, future::Future, path::PathBuf, pin::Pin};

use anyhow::Result;
use clap::{Args, Parser, Subcommand};
use enum_dispatch::enum_dispatch;
use futures::FutureExt;
use pangalactic_hash::Hash;
use pangalactic_host::HostLayerExt;
use pangalactic_layer_cidmeta::{CidMeta, CidMetaLayer};
use pangalactic_layer_dir::LinkDirectoryLayer;
use pangalactic_path::{AnyDestination, AnySource, PathLayerExt, StorePath};
use pangalactic_revcon::ControlDir;
use pangalactic_seed::Seed;
use pangalactic_store::Store;
use pangalactic_store_dirdb::DirDbStore;
use pangalactic_store_mem::MemStore;

type CliStore = LinkDirectoryLayer<CidMetaLayer<DirDbStore>>;

type CliCid = CidMeta<Hash>;

type CliAnyDestination = AnyDestination<CliCid>;
type CliAnySource = AnySource<CliCid>;
type CliStorePath = StorePath<CliCid>;

// Upstream Bug: `enum_dispatch` does not support `async fn` in traits. :-(
#[enum_dispatch]
pub trait Runnable {
    fn run(self) -> RunOutcome;
}

pub type RunOutcome = Pin<Box<dyn Future<Output = Result<Option<Box<dyn Display>>>>>>;

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
    fn run(self) -> RunOutcome {
        use Command::RevCon;
        use RevConCommand::Info;

        self.command
            .unwrap_or(RevCon(Info(RevConInfoOptions { detail: None })))
            .run()
    }
}

#[enum_dispatch(Runnable)]
#[derive(Debug, Subcommand)]
pub enum Command {
    #[command(flatten)]
    RevCon(RevConCommand),
    #[command(subcommand)]
    Util(UtilCommand),
}

/// General Utilities
#[enum_dispatch(Runnable)]
#[derive(Debug, Subcommand)]
pub enum UtilCommand {
    #[command(subcommand, name = "revcon")]
    RevCon(RevConCommand),
    #[command(subcommand)]
    Store(StoreCommand),
    Derive(DeriveOptions),
}

/// Revision Control commands
#[enum_dispatch(Runnable)]
#[derive(Debug, Subcommand)]
pub enum RevConCommand {
    Info(RevConInfoOptions),
    Init(RevConInitOptions),
}

/// RevCon info
#[derive(Debug, Args)]
pub struct RevConInfoOptions {
    #[command(subcommand)]
    pub detail: Option<RevConInfoDetail>,
}

impl Runnable for RevConInfoOptions {
    fn run(self) -> RunOutcome {
        if let Some(detail) = self.detail {
            detail.run()
        } else {
            todo!("info default")
        }
    }
}

/// Revision Control Info subcommands
#[enum_dispatch(Runnable)]
#[derive(Debug, Subcommand)]
pub enum RevConInfoDetail {
    Path(RevConInfoPathOptions),
}

/// Print the control directory path
#[derive(Debug, Args)]
pub struct RevConInfoPathOptions {}

impl Runnable for RevConInfoPathOptions {
    fn run(self) -> RunOutcome {
        Box::pin(async {
            let ctldir = ControlDir::find_from_current_dir()?;
            ok_disp(ctldir)
        })
    }
}

/// Initialize revision control
#[derive(Debug, Args)]
pub struct RevConInitOptions {
    /// The workdir to initialize
    #[clap(long, short, default_value = ".")]
    pub workdir: PathBuf,
}

impl Runnable for RevConInitOptions {
    fn run(self) -> RunOutcome {
        Box::pin(async {
            let ctldir = ControlDir::initialize(self.workdir).await?;
            ok_disp(ctldir)
        })
    }
}

/// Store i/o
#[enum_dispatch(Runnable)]
#[derive(Debug, Subcommand)]
pub enum StoreCommand {
    Put(StorePutOptions),
    Get(StoreGetOptions),
    Xfer(StoreXferOptions),
    #[command(subcommand)]
    Seed(SeedCommand),
}

/// Insert the file on stdin and print its key on stdout
#[derive(Debug, Args)]
pub struct StorePutOptions {}

impl Runnable for StorePutOptions {
    fn run(self) -> RunOutcome {
        Box::pin(async {
            let mut store = CliStore::default();
            store
                .transfer(AnySource::Stdin, AnyDestination::Store(None))
                .map(map_res_disp)
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
    fn run(self) -> RunOutcome {
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
    fn run(self) -> RunOutcome {
        Box::pin(async {
            let mut store = CliStore::default();
            store
                .transfer(self.source, self.dest)
                .map(map_res_disp)
                .await
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
    fn run(self) -> RunOutcome {
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

            let planlink = store.resolve_path(&plan).await?;
            let (_, attestation) = store.derive(&planlink).await?;
            ok_disp(StorePath::from(attestation))
        })
    }
}
/// Manage the pg seed directory
#[enum_dispatch(Runnable)]
#[derive(Debug, Subcommand)]
pub enum SeedCommand {
    List(SeedListOptions),
    Install(SeedInstallOptions),
}

/// List the pgwasm names
#[derive(Debug, Args)]
pub struct SeedListOptions {}

impl Runnable for SeedListOptions {
    fn run(self) -> RunOutcome {
        Box::pin(async {
            let mut store = LinkDirectoryLayer::<MemStore>::default();
            let link = store.commit(Seed).await?;
            todo!("display a manifest of {link:?}")
        })
    }
}

/// Install the stdlib pgwasm directory
#[derive(Debug, Args)]
pub struct SeedInstallOptions {}

impl Runnable for SeedInstallOptions {
    fn run(self) -> RunOutcome {
        Box::pin(async {
            let mut store = CliStore::default();
            let link = store.commit(Seed).await?;
            ok_disp(link)
        })
    }
}

fn ok_disp<T>(value: T) -> Result<Option<Box<dyn Display>>>
where
    T: Display + 'static,
{
    Ok(Some(box_disp(value)))
}

fn map_res_disp<T>(res: Result<Option<T>>) -> Result<Option<Box<dyn Display>>>
where
    T: Display + 'static,
{
    res.map(|opt| opt.map(box_disp))
}

fn box_disp<T>(value: T) -> Box<dyn Display>
where
    T: Display + 'static,
{
    Box::new(value) as Box<dyn Display>
}
