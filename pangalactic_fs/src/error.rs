use std::io::Error;
use std::path::{Path, PathBuf};

pub type Result<T> = std::result::Result<T, PathError>;

#[derive(Debug)]
pub struct PathError {
    path: PathBuf,
    error: Error,
}

impl PathError {
    pub fn wrap_std<P>(p: P) -> impl FnOnce(Error) -> PathError
    where
        P: AsRef<Path>,
    {
        let path = p.as_ref().to_path_buf();

        |error: Error| PathError { path, error }
    }
}

impl From<PathError> for Error {
    fn from(pe: PathError) -> Error {
        pangalactic_errorutil::io_error!(pe.error.kind(), "{:?}: {}", pe.path, pe.error)
    }
}
