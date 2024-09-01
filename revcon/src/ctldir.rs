use std::{
    fmt::Display,
    path::{Path, PathBuf},
};

use anyhow::Result;
use pangalactic_layer_dir::LinkDirectoryLayer;
use pangalactic_linkpath::LinkPath;
use pangalactic_seed::Seed;
use pangalactic_store::Store;

#[derive(Debug, derive_more::From, derive_more::Into)]
pub struct ControlDir(PathBuf);

pub const CONTROL_DIR_NAME: &str = ".pg";

impl ControlDir {
    pub fn find_from_current_dir() -> Result<Self> {
        let cwd = std::env::current_dir()?;
        Self::find_from_path(cwd)
    }

    pub fn find_from_path<P>(startpath: P) -> Result<Self>
    where
        P: AsRef<Path>,
    {
        let sp = startpath.as_ref();
        for p in sp.ancestors() {
            let candidate = p.join(CONTROL_DIR_NAME);
            if candidate.is_dir() {
                return Ok(ControlDir(candidate));
            }
        }
        anyhow::bail!(
            "pg revision control directory not found above {:?}",
            sp.display()
        );
    }

    pub async fn initialize<S, P>(store: &mut LinkDirectoryLayer<S>, workdir: P) -> Result<Self>
    where
        S: Store,
        P: AsRef<Path>,
    {
        use pangalactic_dag_transfer::TransferLayerExt;

        let ctldir = ControlDir(workdir.as_ref().join(".pg"));

        let seed_link = Seed.install(store).await?;
        let template = LinkPath::new(seed_link, "controldir-template")?;

        store.transfer(template, ctldir.0.clone()).await?;

        Ok(ctldir)
    }
}

impl AsRef<Path> for ControlDir {
    fn as_ref(&self) -> &Path {
        self.0.as_ref()
    }
}

impl Display for ControlDir {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        self.0.display().fmt(f)
    }
}
