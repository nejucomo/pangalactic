use std::{
    fmt::Debug,
    future::Future,
    path::{Path, PathBuf},
};

use anyhow::{anyhow, Result};
use pangalactic_iowrappers::{Readable, Writable};
use pangalactic_layer_dir::{LinkDirectory, LinkDirectoryLayer};
use pangalactic_link::Link;
use pangalactic_store::Store;
use tokio::{
    fs::File,
    io::{AsyncRead, AsyncWrite},
};

use crate::{
    fsutil, BranchIter, IntoSource,
    Source::{self, Branch, Leaf},
};

pub trait Sink<S>: Sized + Debug + Send
where
    S: Store,
{
    type CID: Send;

    fn sink<L, B>(self, source: Source<L, B>) -> impl Future<Output = Result<Self::CID>> + Send
    where
        L: Debug + Send + AsyncRead,
        B: Debug + Send + BranchIter<S>,
    {
        async {
            match source {
                Leaf(l) => self.sink_leaf(l).await,
                Branch(b) => self.sink_branch(b).await,
            }
        }
    }

    fn sink_leaf<L>(self, leaf: L) -> impl Future<Output = Result<Self::CID>> + Send
    where
        L: Debug + Send + AsyncRead,
    {
        std::future::ready(Err(anyhow!("{self:?} cannot sink leaf {leaf:?}")))
    }

    fn sink_branch<B>(self, branch: B) -> impl Future<Output = Result<Self::CID>> + Send
    where
        B: Debug + Send + BranchIter<S>,
    {
        std::future::ready(Err(anyhow!("{self:?} cannot sink branch {branch:?}")))
    }
}

pub trait LeafSink {
    type CID;

    fn sink_only_leaf<L>(self, leaf: L) -> impl Future<Output = Result<Self::CID>> + Send
    where
        L: Debug + Send + AsyncRead;
}

impl<'s, S> Sink<S> for &'s mut LinkDirectoryLayer<S>
where
    S: Store,
{
    type CID = Link<S::CID>;

    fn sink_leaf<L>(self, leaf: L) -> impl Future<Output = Result<Self::CID>> + Send
    where
        L: Debug + Send + AsyncRead,
    {
        self.commit(Readable(leaf))
    }

    async fn sink_branch<B>(self, mut branch: B) -> Result<Self::CID>
    where
        B: Debug + Send + BranchIter<S>,
    {
        let mut ld = LinkDirectory::default();

        while let Some((name, intosrc)) = branch.next_branch_entry().await? {
            let src = intosrc.into_source(self).await?;
            let link = self.sink(src).await?;
            ld.insert(name, link)?;
        }

        self.commit(ld).await
    }
}

pub type StoreWith<'s, S, T> = (&'s LinkDirectoryLayer<S>, T);

impl<'s, 'a, S> Sink<S> for StoreWith<'s, S, &'a Path>
where
    S: Store,
{
    type CID = PathBuf;

    fn sink_leaf<L>(self, leaf: L) -> impl Future<Output = Result<Self::CID>> + Send
    where
        L: Debug + Send + AsyncRead,
    {
        let (store, path) = self;
        <StoreWith<'s, S, PathBuf> as Sink<S>>::sink_leaf((store, path.to_path_buf()), leaf)
    }

    fn sink_branch<B>(self, branch: B) -> impl Future<Output = Result<Self::CID>> + Send
    where
        B: Debug + Send + BranchIter<S>,
    {
        let (store, path) = self;
        (store, path.to_path_buf()).sink_branch(branch)
    }
}

impl<'s, S> Sink<S> for StoreWith<'s, S, PathBuf>
where
    S: Store,
{
    type CID = PathBuf;

    async fn sink_leaf<L>(self, leaf: L) -> Result<Self::CID>
    where
        L: Debug + Send + AsyncRead,
    {
        let (_, path) = self;
        let f = fsutil::create_file(&path).await?;
        f.sink_only_leaf(leaf).await?;
        Ok(path)
    }

    async fn sink_branch<B>(self, mut branch: B) -> Result<Self::CID>
    where
        B: Debug + Send + BranchIter<S>,
    {
        let (store, path) = self;
        fsutil::create_dir(&path).await?;
        while let Some((name, intosrc)) = branch.next_branch_entry().await? {
            let subdest = path.join(name);
            let subsrc = intosrc.into_source(store).await?;
            (store, subdest).sink(subsrc).await?;
        }
        Ok(path)
    }
}

impl LeafSink for File {
    type CID = ();

    fn sink_only_leaf<L>(self, leaf: L) -> impl Future<Output = Result<Self::CID>> + Send
    where
        L: Debug + Send + AsyncRead,
    {
        Writable(self).sink_only_leaf(leaf)
    }
}

impl<W> LeafSink for Writable<W>
where
    W: Debug + Send + AsyncWrite,
{
    type CID = ();

    async fn sink_only_leaf<L>(self, leaf: L) -> Result<Self::CID>
    where
        L: Debug + Send + AsyncRead,
    {
        use std::pin::pin;

        tokio::io::copy(&mut pin!(leaf), &mut pin!(self)).await?;
        Ok(())
    }
}
