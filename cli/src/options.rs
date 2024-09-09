use std::{fmt::Display, future::Future, path::PathBuf, pin::Pin};

use anyhow::Result;
use clap::{Args, Parser, Subcommand};
use enum_dispatch::enum_dispatch;
use pangalactic_dag_transfer::TransferLayerExt;
use pangalactic_endpoint::{DestinationEndpoint, SourceEndpoint};
use pangalactic_globset::{Glob, GlobSet};
use pangalactic_hash::Hash;
use pangalactic_host::HostLayerExt;
use pangalactic_layer_cidmeta::{CidMeta, CidMetaLayer};
use pangalactic_layer_dir::LinkDirectoryLayer;
use pangalactic_manifest::FullManifest;
use pangalactic_revcon::ControlDir;
use pangalactic_seed::Seed;
use pangalactic_store::Store;
use pangalactic_store_dirdb::DirDbStore;
use pangalactic_store_mem::MemStore;

type CliStore = LinkDirectoryLayer<CidMetaLayer<DirDbStore>>;
type CliCid = CidMeta<Hash>;
type CliDestinationEndpoint = DestinationEndpoint<CliCid>;
type CliSourceEndpoint = SourceEndpoint<CliCid>;

#[enum_dispatch]
pub trait Runnable {
    fn run(self) -> RunOutcome;
}

// We must use `Box<Pin<_>>` to satisfy `enum_dispatch`. :-/
pub type RunOutcome = Pin<Box<dyn Future<Output = Result<()>>>>;

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
            let mut store = CliStore::default();
            let ctldir = ControlDir::initialize(&mut store, self.workdir).await?;
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
            let link = store.transfer(SourceEndpoint::mk_stdin(), ()).await?;
            ok_disp(link)
        })
    }
}

/// Send the given file to stdout
#[derive(Debug, Args)]
pub struct StoreGetOptions {
    /// The source to get
    pub source: CliSourceEndpoint,
}

impl Runnable for StoreGetOptions {
    fn run(self) -> RunOutcome {
        Box::pin(async {
            let mut store = CliStore::default();
            store
                .transfer(self.source, DestinationEndpoint::for_stdout())
                .await?;
            Ok(())
        })
    }
}

/// Transfer from SOURCE to DEST
#[derive(Debug, Args)]
pub struct StoreXferOptions {
    #[clap(flatten)]
    pub excludes: ExcludeGlobOptions,

    pub source: CliSourceEndpoint,
    pub dest: CliDestinationEndpoint,
}

impl Runnable for StoreXferOptions {
    fn run(self) -> RunOutcome {
        Box::pin(async {
            let mut store = CliStore::default();
            let globset = self.excludes.into_globset()?;
            let source = globset.filter_source(self.source);
            let receipt = store.transfer(source, self.dest).await?;
            if receipt.is_stdout() {
                Ok(())
            } else {
                ok_disp(receipt)
            }
        })
    }
}

/// Derive a plan
#[derive(Debug, Args)]
pub struct DeriveOptions {
    /// The plan to derive, or an exec file if `INPUT` is provided
    pub plan_or_exec: CliSourceEndpoint,

    /// An input to derive; if absent `PLAN_OR_EXEC` must be a plan
    pub input: Option<CliSourceEndpoint>,
}

impl Runnable for DeriveOptions {
    fn run(self) -> RunOutcome {
        Box::pin(async {
            use pangalactic_schemata::Plan;

            let mut store = CliStore::default();

            // Transfer any source into the store to get a store path:
            // Assert: Final unwrap never fails because `DestinationEndpoint::Store` always produces a path:
            let exec = store.transfer(self.plan_or_exec, ()).await?;

            let plan = if let Some(input) = self.input {
                let input = store.transfer(input, ()).await?;
                store.commit(Plan { exec, input }).await?
            } else {
                exec
            };

            let (_, attestation) = store.derive(&plan).await?;
            ok_disp(attestation)
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
            let mani: FullManifest<_> = store.load(&link).await?;
            ok_disp(mani)
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
            let link = Seed.install(&mut store).await?;
            ok_disp(link)
        })
    }
}

#[derive(Clone, Debug, Args)]
pub struct ExcludeGlobOptions {
    /// Exclude the given glob pattern (multiple repetitions allowed)
    #[clap(long, short)]
    exclude: Vec<Glob>,
}

impl ExcludeGlobOptions {
    pub fn into_globset(self) -> Result<GlobSet> {
        GlobSet::try_from(self.exclude)
    }
}

fn ok_disp<T>(value: T) -> Result<()>
where
    T: Display,
{
    println!("{value}");
    Ok(())
}
