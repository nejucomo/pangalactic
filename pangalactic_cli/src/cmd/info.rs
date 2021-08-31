#[cfg(test)]
mod tests;

use std::io::Result;
use std::path::Path;

pub fn info<W>(w: W, path: &Path) -> Result<()>
where
    W: std::io::Write,
{
    let repo = crate::repo::Repo::find_from(path)?;
    serde_json::to_writer_pretty(w, &repo).unwrap();
    Ok(())
}
