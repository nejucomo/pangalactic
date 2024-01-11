use crate::tests::dagcase::DagCase;
use anyhow_std::PathAnyhow;
use async_trait::async_trait;
use std::{
    borrow::Cow,
    path::{Path, PathBuf},
};

#[derive(Debug, PartialEq)]
pub struct PathVerifier(Cow<'static, str>);

pub const fn mkpv(s: &'static str) -> PathVerifier {
    PathVerifier(Cow::Borrowed(s))
}

fn tempdir_name_prefix() -> String {
    format!("{}_testdata.", env!("CARGO_PKG_NAME"))
}

#[async_trait]
impl DagCase for PathBuf {
    type Ctr = &'static [&'static str];
    type Verifier = PathVerifier;

    async fn setup(constructor: Self::Ctr) -> anyhow::Result<Self> {
        let testdir = tempfile::TempDir::with_prefix(&tempdir_name_prefix())?.into_path();

        for filepathstr in constructor {
            let filepath = testdir.join(filepathstr);

            filepath
                .parent_anyhow()?
                .create_dir_all_anyhow()
                .or_else(|e| {
                    use std::io::{Error, ErrorKind::AlreadyExists};

                    if e.downcast_ref::<Error>()
                        .map(|stderr| stderr.kind() == AlreadyExists)
                        .unwrap_or(false)
                    {
                        // It's ok that the directory already exists:
                        Ok(())
                    } else {
                        // Propagate other errors:
                        Err(e)
                    }
                })?;

            filepath.write_anyhow(format!("contents of {filepathstr:?}"))?;
        }
        Ok(testdir)
    }

    async fn cleanup(self) -> anyhow::Result<()> {
        tokio::fs::remove_dir_all(self).await?;
        Ok(())
    }
}

impl From<PathBuf> for PathVerifier {
    fn from(path: PathBuf) -> Self {
        try_from(&path).unwrap()
    }
}

fn try_from(path: &Path) -> anyhow::Result<PathVerifier> {
    let basepath = find_basedir(path)?;
    let suffix = path.strip_prefix_anyhow(basepath)?;
    let s = suffix.to_str_anyhow()?;
    Ok(PathVerifier(Cow::from(s.to_string())))
}

fn find_basedir(mut p: &Path) -> anyhow::Result<&Path> {
    use anyhow_std::OsStrAnyhow;

    let pref = tempdir_name_prefix();

    loop {
        if p.file_name_anyhow()?.to_str_anyhow()?.starts_with(&pref) {
            return Ok(p);
        } else {
            p = p.parent_anyhow()?;
        }
    }
}
