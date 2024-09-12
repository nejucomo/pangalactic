use std::{fmt::Debug, fmt::Display, str::FromStr};

use anyhow::Result;
use pangalactic_dag_transfer::{BranchIter, Destination, LeafDestination};
use pangalactic_iowrappers::Readable;
use pangalactic_layer_dir::{LinkDirectory, LinkDirectoryLayer};
use pangalactic_link::Link;
use pangalactic_name::{NonEmptyPath, NonEmptyPathRef, Path};
use pangalactic_store::{Commit, Store};
use serde::{de::DeserializeOwned, Serialize};
use tokio::io::AsyncRead;

use crate::LinkPath;

#[derive(Clone, Eq, PartialEq)]
pub(super) struct Inner<C> {
    /// Invariant: self.link.kind() == Dir
    link: Link<C>,
    path: NonEmptyPath,
}

impl<C> Inner<C> {
    pub(super) fn new<P>(link: Link<C>, path: P) -> Result<Self>
    where
        NonEmptyPath: TryFrom<P>,
        <NonEmptyPath as TryFrom<P>>::Error: std::error::Error + Send + Sync + 'static,
    {
        use pangalactic_linkkind::LinkKind::Dir;

        // Ensure this is a Dir link:
        link.peek_cid_kind(Dir)?;
        let path = NonEmptyPath::try_from(path)?;

        Ok(Inner { link, path })
    }

    pub(super) fn link(&self) -> &Link<C> {
        &self.link
    }

    pub(super) fn path(&self) -> &NonEmptyPathRef {
        self.path.as_ref()
    }

    pub(super) async fn commit_within<S, T>(
        self,
        store: &mut LinkDirectoryLayer<S>,
        value: T,
    ) -> Result<LinkPath<C>>
    where
        C: Clone + Send + Serialize,
        S: Store<CID = C>,
        T: Commit<LinkDirectoryLayer<S>> + Send,
    {
        let mut link = store.commit(value).await?;

        let mut dirlink = self.link().clone();
        let mut stack = vec![];
        let (intermediate, last) = self.path().split_last();

        for name in intermediate.components() {
            let d: LinkDirectory<S::CID> = store.load(&dirlink).await?;
            dirlink = d.get_required(name)?.clone();
            stack.push((d, name));
        }

        let mut d: LinkDirectory<S::CID> = store.load(&dirlink).await?;
        d.insert(last.to_owned(), link)?;

        for (mut prevd, name) in stack.into_iter().rev() {
            link = store.commit(d).await?;
            prevd.overwrite(name.to_owned(), link)?;
            d = prevd;
        }

        let newroot = store.commit(d).await?;
        self.replace_link_into_path(newroot)
    }

    pub(super) fn replace_link_into_path(self, newroot: Link<C>) -> Result<LinkPath<C>>
    where
        C: Serialize,
    {
        LinkPath::new(newroot, Path::from(self.path))
    }
}

impl<S> Destination<S> for Inner<S::CID>
where
    S: Store,
{
    async fn sink_branch<B>(self, store: &mut LinkDirectoryLayer<S>, branch: B) -> Result<Self::CID>
    where
        B: Debug + Send + BranchIter<S>,
    {
        self.commit_within(store, branch).await
    }
}

impl<S> LeafDestination<S> for Inner<S::CID>
where
    S: Store,
{
    type CID = LinkPath<S::CID>;

    async fn sink_leaf<L>(self, store: &mut LinkDirectoryLayer<S>, leaf: L) -> Result<Self::CID>
    where
        L: Debug + Send + AsyncRead,
    {
        self.commit_within(store, Readable(leaf)).await
    }
}

impl<C> Display for Inner<C>
where
    C: Serialize,
{
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "{}/{}", self.link, self.path)
    }
}

impl<C> Debug for Inner<C>
where
    C: Serialize,
{
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        f.debug_struct(std::any::type_name::<Self>())
            .field("link", &self.link)
            .field("path", &self.path)
            .finish()
    }
}

impl<C> FromStr for Inner<C>
where
    C: DeserializeOwned + Serialize,
{
    type Err = anyhow::Error;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        let lp: LinkPath<C> = s.parse()?;
        let (link, path) = lp.into();
        let nep = NonEmptyPath::try_from(path)?;
        Self::new(link, nep)
    }
}
