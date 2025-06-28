use std::{
    fmt::Display,
    path::{Path, PathBuf},
};

use anyhow::Result;
use anyhow_std::PathAnyhow as _;
use derive_more::Constructor;
use pangalactic_dag_transfer::TransferLayerExt as _;
use pangalactic_layer_dir::LinkDirectoryLayer;
use pangalactic_linkpath::LinkPath;
use pangalactic_store::Store;

const SEED_LINK: &str = include_str!(env!("PANGALACTIC_SEED_LINK_PATH"));
const BOOKKEEPING_TEMPLATE_NAME: &str = "bookkeeping-template";

/// A workspace is a working directory with bookkeeping metadata which can record revisions
#[derive(Debug, Constructor)]
pub struct Workspace<S>
where
    S: Store,
{
    /// The store
    store: LinkDirectoryLayer<S>,

    /// The book-keeping directory
    bkdir: PathBuf,
}

/// The name of the "bookkeeping" directory of a workspace
pub const BOOKKEEPING_DIR_NAME: &str = ".pg";

impl<S> Workspace<S>
where
    S: Store,
{
    /// The path to this [Workspace]
    pub fn path(&self) -> &Path {
        self.bkdir.parent_anyhow().unwrap()
    }

    /// Equivalent to [Workspace::find_from_path] with the current directory
    pub async fn find_from_current_dir(store: LinkDirectoryLayer<S>) -> Result<Self> {
        let cwd = std::env::current_dir()?;
        Self::find_from_path(store, cwd).await
    }

    /// If the given path is within a [Workspace], return that workspace; otherwise [Err]
    pub async fn find_from_path<P>(store: LinkDirectoryLayer<S>, startpath: P) -> Result<Self>
    where
        P: AsRef<Path>,
    {
        let sp = startpath.as_ref();
        for p in sp.ancestors() {
            let candidate = p.join(BOOKKEEPING_DIR_NAME);
            if candidate.is_dir() {
                return Ok(Workspace::new(store, candidate));
            }
        }
        anyhow::bail!("pg workspace root not found above {:?}", sp.display());
    }

    /// Initialize a path as a new [Workspace]
    pub async fn initialize<P>(store: LinkDirectoryLayer<S>, workdir: P) -> Result<Self>
    where
        S: Store,
        P: AsRef<Path>,
    {
        let mut ws = Workspace::new(store, workdir.as_ref().join(BOOKKEEPING_DIR_NAME));
        let template: LinkPath<S::CID> =
            format!("{}/{BOOKKEEPING_TEMPLATE_NAME}", SEED_LINK.trim_end()).parse()?;
        ws.store.transfer(template, ws.path().to_path_buf()).await?;

        Ok(ws)
    }
}

impl<S> Display for Workspace<S>
where
    S: Store,
{
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        self.path().display().fmt(f)
    }
}
