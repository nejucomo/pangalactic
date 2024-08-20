// TODO: replace with a `anyhow-tokio` crate

use std::path::Path;

use anyhow::{Context, Result};
use tokio::fs;

pub(crate) async fn create_dir<P>(p: P) -> Result<()>
where
    P: AsRef<Path>,
{
    let pref = p.as_ref();
    fs::create_dir(pref)
        .await
        .with_context(|| format!("while creating directory {:?}", pref.display()))
}
