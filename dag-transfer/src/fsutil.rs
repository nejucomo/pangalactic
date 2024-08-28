use std::path::Path;

use anyhow::{Context, Result};
use tokio::fs::File;

pub(crate) async fn create_file<P>(path: P) -> Result<File>
where
    P: AsRef<Path>,
{
    let path = path.as_ref();

    tracing::debug!(?path, "creating file");
    File::create_new(path)
        .await
        .with_context(|| format!("while creating file {:?}", path.display()))
}

pub(crate) async fn create_dir<P>(path: P) -> Result<()>
where
    P: AsRef<Path>,
{
    let path = path.as_ref();

    tracing::debug!(?path, "creating directory");
    tokio::fs::create_dir(path)
        .await
        .with_context(|| format!("while creating directory {:?}", path.display()))
}
