use std::path::{Path, PathBuf};

use crate::PgDirs;

pub fn get<P>(name: P) -> PathBuf
where
    P: AsRef<Path>,
{
    PgDirs::singleton().data.join(name)
}
