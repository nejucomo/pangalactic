#[cfg(test)]
mod tests;

use crate::repo::Repo;
use std::io::Result;
use std::path::Path;

pub fn info(path: &Path) -> Result<Repo> {
    Repo::find_from(path)
}
