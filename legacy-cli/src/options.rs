use std::{fmt::Display, future::Future, path::PathBuf, pin::Pin};

use anyhow::Result;
use clap::{Args, Parser, Subcommand};
use enum_dispatch::enum_dispatch;
use pangalactic_dag_transfer::TransferLayerExt;
use pangalactic_endpoint::{DestinationEndpoint, Endpoint, OriginEndpoint, Stdio};
use pangalactic_globset::{Glob, GlobSet};
use pangalactic_hash::Hash;
use pangalactic_host::HostLayerExt;
use pangalactic_layer_cidmeta::{CidMeta, CidMetaLayer};
use pangalactic_layer_dir::LinkDirectoryLayer;
use pangalactic_revcon::ControlDir;
use pangalactic_store::Store;
use pangalactic_store_dirdb::DirDbStore;

type CliStore = LinkDirectoryLayer<CidMetaLayer<DirDbStore>>;
type CliCid = CidMeta<Hash>;
type CliDestinationEndpoint = DestinationEndpoint<CliCid>;
type CliOriginEndpoint = OriginEndpoint<CliCid>;

#[enum_dispatch]
pub trait Runnable {
    fn run(self) -> RunOutcome;
}

// We must use `Box<Pin<_>>` to satisfy `enum_dispatch`. :-/
pub type RunOutcome = Pin<Box<dyn Future<Output = Result<()>>>>;

fn ok_disp<T>(value: T) -> Result<()>
where
    T: Display,
{
    println!("{value}");
    Ok(())
}
