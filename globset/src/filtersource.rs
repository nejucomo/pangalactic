use anyhow::Result;
use pangalactic_dag_transfer::{BranchIter, IntoSource, Source};
use pangalactic_layer_dir::LinkDirectoryLayer;
use pangalactic_name::{Name, Path};
use pangalactic_store::{Commit, Store};

use crate::GlobSet;

#[derive(Debug)]
pub struct FilterSource<'g, T> {
    globset: &'g GlobSet,
    path: Path,
    inner: T,
}

impl<'g, T> FilterSource<'g, T> {
    pub(crate) fn new(globset: &'g GlobSet, inner: T) -> Self {
        Self::new_with_path(globset, inner, Path::default())
    }

    pub(crate) fn new_with_path(globset: &'g GlobSet, inner: T, path: Path) -> Self {
        Self {
            globset,
            inner,
            path,
        }
    }
}

impl<'g, I, S> IntoSource<S> for FilterSource<'g, I>
where
    S: Store,
    I: IntoSource<S>,
{
    type Leaf = I::Leaf;
    type Branch = FilterSource<'g, I::Branch>;

    async fn into_source(
        self,
        store: &LinkDirectoryLayer<S>,
    ) -> Result<Source<Self::Leaf, Self::Branch>> {
        use Source::*;

        let FilterSource {
            globset,
            path,
            inner,
        } = self;

        if globset.is_match(&path) {
            anyhow::bail!("{globset:?} filters out {path:?}");
        } else {
            match inner.into_source(store).await? {
                Leaf(l) => Ok(Leaf(l)),
                Branch(b) => Ok(Branch(FilterSource::new_with_path(globset, b, path))),
            }
        }
    }
}

impl<'g, B, S> BranchIter<S> for FilterSource<'g, B>
where
    S: Store,
    B: BranchIter<S>,
{
    type IntoSource = FilterSource<'g, B::IntoSource>;

    async fn next_branch_entry(&mut self) -> Result<Option<(Name, Self::IntoSource)>> {
        while let Some((name, intosrc)) = self.inner.next_branch_entry().await? {
            let subpath = self.path.join(&name);
            if self.globset.is_match(&subpath) {
                tracing::debug!(?subpath, "filtered out");
            } else {
                let wrapper = FilterSource::new_with_path(self.globset, intosrc, subpath);
                let item = (name, wrapper);
                return Ok(Some(item));
            }
        }
        Ok(None)
    }
}

impl<'g, B, S> Commit<LinkDirectoryLayer<S>> for FilterSource<'g, B>
where
    S: Store,
    B: BranchIter<S>,
{
    async fn commit_into_store(
        self,
        store: &mut LinkDirectoryLayer<S>,
    ) -> Result<<LinkDirectoryLayer<S> as Store>::CID> {
        store.commit(self.inner).await
    }
}
