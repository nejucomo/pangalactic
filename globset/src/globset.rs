use std::{fmt::Debug, path::Path};

use anyhow::Result;
pub use globset::Glob;
use globset::GlobSet as Upstream;
use serde::{Deserialize, Serialize};

use crate::filtersource::FilterSource;

#[derive(Clone, Deserialize, Serialize)]
#[serde(try_from = "Vec<&str>", into = "Vec<String>")]
pub struct GlobSet {
    globs: Vec<Glob>,
    matcher: Upstream,
}

impl GlobSet {
    pub fn filter_source<I>(&self, intosource: I) -> FilterSource<I> {
        FilterSource::new(self, intosource)
    }

    pub fn is_match<P>(&self, path: P) -> bool
    where
        P: AsRef<Path>,
    {
        self.matcher.is_match(path)
    }
}

impl<'a> TryFrom<Vec<&'a str>> for GlobSet {
    type Error = anyhow::Error;

    fn try_from(vec: Vec<&'a str>) -> Result<Self> {
        Self::try_from(vec.as_slice())
    }
}

impl<'a, 'b> TryFrom<&'b [&'a str]> for GlobSet {
    type Error = anyhow::Error;

    fn try_from(globsrcs: &'b [&'a str]) -> Result<Self> {
        let v: Vec<Glob> = globsrcs
            .iter()
            .map(|src| src.parse::<Glob>())
            .collect::<std::result::Result<_, _>>()?;

        GlobSet::try_from(v)
    }
}

impl TryFrom<Vec<Glob>> for GlobSet {
    type Error = anyhow::Error;

    fn try_from(globs: Vec<Glob>) -> Result<Self> {
        let mut b = Upstream::builder();
        for g in globs.iter() {
            b.add(g.clone());
        }
        let matcher = b.build()?;
        Ok(GlobSet { globs, matcher })
    }
}

impl From<GlobSet> for Vec<String> {
    fn from(gs: GlobSet) -> Self {
        gs.globs.into_iter().map(|g| g.glob().to_string()).collect()
    }
}

impl Debug for GlobSet {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        f.debug_struct("GlobSet")
            .field("globs", &self.globs)
            .field("matcher", &"...")
            .finish()
    }
}
