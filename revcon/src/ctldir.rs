use std::path::{Path, PathBuf};

use anyhow::Result;

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

    pub async fn initialize<P>(workdir: P) -> Result<Self>
    where
        P: AsRef<Path>,
    {
        dbg!(workdir.as_ref());
        todo!()
    }
}

impl AsRef<Path> for ControlDir {
    fn as_ref(&self) -> &Path {
        self.0.as_ref()
    }
}
