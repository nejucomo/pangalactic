use serde::{Deserialize, Serialize};
use std::io::Result;
use std::path::{Path, PathBuf};

#[derive(Debug, PartialEq, Eq, Deserialize, Serialize)]
pub struct Repo {
    basedir: PathBuf,
}

impl<P> From<P> for Repo
where
    P: AsRef<Path>,
{
    fn from(p: P) -> Repo {
        Repo::wrap(p.as_ref().to_path_buf())
    }
}

impl Repo {
    pub fn wrap(basedir: PathBuf) -> Repo {
        Repo { basedir }
    }

    pub fn find_from<P>(subpath: P) -> Result<Repo>
    where
        P: AsRef<Path>,
    {
        use crate::PG_REPO_CONTROL;
        use pangalactic_errorutil::io_error;

        let spref = subpath.as_ref();

        for ancestor in spref.canonicalize()?.ancestors() {
            if ancestor.join(PG_REPO_CONTROL).is_dir() {
                return Ok(Repo {
                    basedir: ancestor.to_path_buf(),
                });
            }
        }

        return Err(io_error!(
            std::io::ErrorKind::NotFound,
            "Path {:?} does not appear to be inside a pangalactic repository.",
            spref
        ));
    }
}
