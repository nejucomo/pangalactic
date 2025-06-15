use std::fmt::Debug;

use anyhow::Result;
pub use globset::Glob;
use globset::GlobSet as Upstream;
use pangalactic_endpoint::OriginEndpoint;
use pangalactic_name::PathRef;
use serde::{Deserialize, Serialize};

use crate::FilteredOrigin;

#[derive(Clone, Deserialize, Serialize)]
#[serde(try_from = "Vec<&str>", into = "Vec<String>")]
pub struct GlobSet {
    globs: Vec<Glob>,
    matcher: Upstream,
}

impl GlobSet {
    pub fn filter_source<C>(&self, origin: OriginEndpoint<C>) -> FilteredOrigin<'_, C>
    where
        C: Serialize,
    {
        FilteredOrigin::new(self, origin)
    }

    pub fn matches<C>(&self, origin: &OriginEndpoint<C>) -> bool {
        if let Some(path) = origin_to_opt_path(origin) {
            self.matcher.is_match(path)
        } else {
            false
        }
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

fn origin_to_opt_path<C>(origin: &OriginEndpoint<C>) -> Option<&PathRef> {
    origin.as_ref().project_into(
        |_| None,
        |p| PathRef::opt_from_std_path(p.as_ref()),
        |p| Some(p.path()),
    )
}
