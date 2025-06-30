use std::{
    fmt::Display,
    path::{Path, PathBuf},
};

use anyhow::Result;
use anyhow_std::PathAnyhow as _;
use derive_more::Constructor;
use pangalactic_dag_transfer::TransferLayerExt as _;
use pangalactic_layer_dir::LinkDirectoryLayer;
use pangalactic_store::Store;

use crate::RevConConfig;

/// A workspace is a working directory with bookkeeping metadata which can record revisions
#[derive(Debug, Constructor)]
pub struct Workspace<S>
where
    S: Store,
{
    /// App config:
    appconfig: RevConConfig<S>,

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
    pub async fn find_from_current_dir(
        appconfig: RevConConfig<S>,
        store: LinkDirectoryLayer<S>,
    ) -> Result<Self> {
        let cwd = std::env::current_dir()?;
        Self::find_from_path(appconfig, store, cwd).await
    }

    /// If the given path is within a [Workspace], return that workspace; otherwise [Err]
    pub async fn find_from_path<P>(
        appconfig: RevConConfig<S>,
        store: LinkDirectoryLayer<S>,
        startpath: P,
    ) -> Result<Self>
    where
        P: AsRef<Path>,
    {
        let sp = startpath.as_ref();
        for p in sp.ancestors() {
            let candidate = p.join(BOOKKEEPING_DIR_NAME);
            if candidate.is_dir() {
                return Ok(Workspace::new(appconfig, store, candidate));
            }
        }
        anyhow::bail!("pg workspace root not found above {:?}", sp.display());
    }

    /// Initialize a path as a new [Workspace]
    pub async fn initialize<P>(
        appconfig: RevConConfig<S>,
        store: LinkDirectoryLayer<S>,
        workdir: P,
    ) -> Result<Self>
    where
        S: Store,
        P: AsRef<Path>,
    {
        let mut workspace = Workspace::new(
            appconfig,
            store,
            workdir.as_ref().join(BOOKKEEPING_DIR_NAME),
        );
        tracing::debug!(?workspace, "initializing");
        workspace.bkdir.create_dir_anyhow()?;

        if let Some(template) = workspace.appconfig.template.clone() {
            tracing::debug!(?template);
            workspace
                .store
                .transfer(template, workspace.bkdir.clone())
                .await?;
        }

        Ok(workspace)
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
