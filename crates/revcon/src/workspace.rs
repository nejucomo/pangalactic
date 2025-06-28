use std::{
    fmt::Display,
    path::{Path, PathBuf},
};

use anyhow::Result;
use anyhow_std::PathAnyhow as _;
use pangalactic_config::Configuration as _;
use pangalactic_layer_dir::LinkDirectoryLayer;
use pangalactic_linkpath::LinkPath;
use pangalactic_store::Store;

use crate::RevConConfig;

/// A workspace is a working directory with bookkeeping metadata which can record revisions
#[derive(Debug)]
pub struct Workspace<S>
where
    S: Store,
{
    config: RevConConfig<S>,
    /// The book-keeping directory
    bkdir: PathBuf,
}

/// The name of the "bookkeeping" directory of a workspace
pub const BOOKKEEPING_DIR_NAME: &str = ".pg";

impl<S> Workspace<S>
where
    S: Store,
{
    async fn new(bkdir: PathBuf) -> Result<Self> {
        let config = RevConConfig::load().await?;
        Ok(Workspace { config, bkdir })
    }

    /// The path to this [Workspace]
    pub fn path(&self) -> &Path {
        self.bkdir.parent_anyhow().unwrap()
    }

    /// Equivalent to [Workspace::find_from_path] with the current directory
    pub async fn find_from_current_dir() -> Result<Self> {
        let cwd = std::env::current_dir()?;
        Self::find_from_path(cwd).await
    }

    /// If the given path is within a [Workspace], return that workspace; otherwise [Err]
    pub async fn find_from_path<P>(startpath: P) -> Result<Self>
    where
        P: AsRef<Path>,
    {
        let sp = startpath.as_ref();
        for p in sp.ancestors() {
            let candidate = p.join(BOOKKEEPING_DIR_NAME);
            if candidate.is_dir() {
                return Workspace::new(candidate).await;
            }
        }
        anyhow::bail!("pg workspace root not found above {:?}", sp.display());
    }

    /// Initialize a path as a new [Workspace]
    pub async fn initialize<P>(store: &mut LinkDirectoryLayer<S>, workdir: P) -> Result<Self>
    where
        S: Store,
        P: AsRef<Path>,
    {
        use pangalactic_dag_transfer::TransferLayerExt;

        let ws = Workspace::new(workdir.as_ref().join(BOOKKEEPING_DIR_NAME)).await?;
        let template = LinkPath::new(ws.config.seed.clone(), "controldir-template")?;
        store.transfer(template, ws.path().to_path_buf()).await?;

        Ok(ws)
    }
}

impl<S> Display for Workspace<S>
where
    S: Store,
{
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        self.bkdir.display().fmt(f)
    }
}
