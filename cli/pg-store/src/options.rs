use anyhow::Result;
use clap::{Args, Parser, Subcommand};
use pangalactic_globset::{Glob, GlobSet};
use pangalactic_std_store::{StdDestination, StdOrigin};

/// Store i/o
#[derive(Debug, Parser)]
pub struct StoreOptions {
    #[clap(subcommand)]
    pub command: StoreCommand,
}

#[derive(Debug, Subcommand)]
pub enum StoreCommand {
    Put(StorePutOptions),
    Get(StoreGetOptions),
    Xfer(StoreXferOptions),
}

/// Insert the file on stdin and print its key on stdout
#[derive(Debug, Args)]
pub struct StorePutOptions {}

/// Send the given file to stdout
#[derive(Debug, Args)]
pub struct StoreGetOptions {
    /// The source to get
    pub source: StdOrigin,
}

/// Transfer from SOURCE to DEST
#[derive(Clone, Debug, Args)]
pub struct StoreXferOptions {
    #[clap(flatten)]
    pub excludes: ExcludeGlobOptions,

    pub source: StdOrigin,
    pub dest: StdDestination,
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
