use std::path::{Path, PathBuf};

use anyhow::Result;
use anyhow_std::PathAnyhow;

pub fn setup(dataset: &str) -> Result<PathBuf> {
    let testcasedir = Path::new("target").join("testdata").join(dataset);

    dbg!(&testcasedir);

    testcasedir.remove_dir_all_anyhow().or_else(|e| {
        use std::io::ErrorKind::NotFound;

        match e.downcast_ref::<std::io::Error>() {
            Some(e) if e.kind() == NotFound => Ok(()),
            _ => Err(e),
        }
    })?;

    testcasedir.create_dir_all_anyhow()?;

    Ok(testcasedir)
}
