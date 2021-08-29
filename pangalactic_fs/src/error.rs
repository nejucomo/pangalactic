use std::io::Error;
use std::path::{Path, PathBuf};

pub type Result<T> = std::result::Result<T, PathError>;

#[derive(Debug)]
pub struct PathError {
    pathinfo: PathInfo,
    error: Error,
}

#[derive(Debug)]
pub enum PathInfo {
    Unary(PathBuf),
    Binary(PathBuf, PathBuf),
}

impl PathError {
    pub fn wrap_std<P>(p: P) -> impl FnOnce(Error) -> PathError
    where
        P: AsRef<Path>,
    {
        PathError::wrap_pathinfo(PathInfo::Unary(p.as_ref().to_path_buf()))
    }

    pub fn wrap_std2<P, Q>(p: P, q: Q) -> impl FnOnce(Error) -> PathError
    where
        P: AsRef<Path>,
        Q: AsRef<Path>,
    {
        PathError::wrap_pathinfo(PathInfo::Binary(
            p.as_ref().to_path_buf(),
            q.as_ref().to_path_buf(),
        ))
    }

    pub fn wrap_pathinfo(pathinfo: PathInfo) -> impl FnOnce(Error) -> PathError {
        |error: Error| PathError { pathinfo, error }
    }
}

impl From<PathError> for Error {
    fn from(pe: PathError) -> Error {
        use pangalactic_errorutil::io_error;
        use PathInfo::*;

        let kind = pe.error.kind();

        match pe.pathinfo {
            Unary(p) => io_error!(kind, "{:?}: {}", p, pe.error),
            Binary(p, q) => io_error!(kind, "{:?} -> {:?}: {}", p, q, pe.error),
        }
    }
}
