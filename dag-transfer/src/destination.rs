use std::{fmt::Debug, future::Future, path::PathBuf};

use anyhow::Result;
use pangalactic_iowrappers::{Readable, Writable};
use pangalactic_layer_dir::{LinkDirectory, LinkDirectoryLayer};
use pangalactic_link::Link;
use pangalactic_store::Store;
use tokio::{
    fs::File,
    io::{AsyncRead, AsyncWrite, Stdout},
};

use crate::{
    fsutil, BranchIter, IntoSource,
    Source::{self, Branch, Leaf},
};

pub trait Destination<S>: Sized + Send + Debug + LeafDestination<S>
where
    S: Store,
{
    fn sink<L, B>(
        self,
        store: &mut LinkDirectoryLayer<S>,
        source: Source<L, B>,
    ) -> impl Future<Output = Result<Self::CID>> + Send
    where
        L: Debug + Send + AsyncRead,
        B: Debug + Send + BranchIter<S>,
    {
        async {
            match source {
                Leaf(l) => self.sink_leaf(store, l).await,

                // `Box::pin` impacts every `Destination` but without this there is a layout-cycle whackamole:
                Branch(b) => Box::pin(self.sink_branch(store, b)).await,
            }
        }
    }

    fn sink_branch<B>(
        self,
        store: &mut LinkDirectoryLayer<S>,
        branch: B,
    ) -> impl Future<Output = Result<Self::CID>> + Send
    where
        B: Debug + Send + BranchIter<S>;
}

pub trait LeafDestination<S>
where
    S: Store,
{
    type CID: Send;

    fn sink_leaf<L>(
        self,
        store: &mut LinkDirectoryLayer<S>,
        leaf: L,
    ) -> impl Future<Output = Result<Self::CID>> + Send
    where
        L: Debug + Send + AsyncRead;
}

impl<S> Destination<S> for ()
where
    S: Store,
{
    async fn sink_branch<B>(
        self,
        store: &mut LinkDirectoryLayer<S>,
        mut branch: B,
    ) -> Result<Self::CID>
    where
        B: Debug + Send + BranchIter<S>,
    {
        let mut ld = LinkDirectory::default();

        while let Some((name, intosrc)) = branch.next_branch_entry().await? {
            let src = intosrc.into_source(store).await?;
            let link = self.sink(store, src).await?;
            ld.insert(name, link)?;
        }

        store.commit(ld).await
    }
}

impl<S> LeafDestination<S> for ()
where
    S: Store,
{
    type CID = Link<S::CID>;

    fn sink_leaf<L>(
        self,
        store: &mut LinkDirectoryLayer<S>,
        leaf: L,
    ) -> impl Future<Output = Result<Self::CID>> + Send
    where
        L: Debug + Send + AsyncRead,
    {
        store.commit(Readable(leaf))
    }
}

impl<S> LeafDestination<S> for PathBuf
where
    S: Store,
{
    type CID = PathBuf;

    async fn sink_leaf<L>(self, store: &mut LinkDirectoryLayer<S>, leaf: L) -> Result<Self::CID>
    where
        L: Debug + Send + AsyncRead,
    {
        let f = fsutil::create_file(&self).await?;
        f.sink_leaf(store, leaf).await?;
        Ok(self)
    }
}

impl<S> Destination<S> for PathBuf
where
    S: Store,
{
    async fn sink_branch<B>(
        self,
        store: &mut LinkDirectoryLayer<S>,
        mut branch: B,
    ) -> Result<Self::CID>
    where
        B: Debug + Send + BranchIter<S>,
    {
        fsutil::create_dir(&self).await?;
        while let Some((name, intosrc)) = branch.next_branch_entry().await? {
            let subsrc = intosrc.into_source(store).await?;
            self.join(name).sink(store, subsrc).await?;
        }
        Ok(self)
    }
}

impl<S> LeafDestination<S> for File
where
    S: Store,
{
    type CID = ();

    fn sink_leaf<L>(
        self,
        store: &mut LinkDirectoryLayer<S>,
        leaf: L,
    ) -> impl Future<Output = Result<Self::CID>> + Send
    where
        L: Debug + Send + AsyncRead,
    {
        Writable(self).sink_leaf(store, leaf)
    }
}

impl<S> LeafDestination<S> for Stdout
where
    S: Store,
{
    type CID = ();

    fn sink_leaf<L>(
        self,
        store: &mut LinkDirectoryLayer<S>,
        leaf: L,
    ) -> impl Future<Output = Result<Self::CID>> + Send
    where
        L: Debug + Send + AsyncRead,
    {
        Writable(self).sink_leaf(store, leaf)
    }
}

impl<S, W> LeafDestination<S> for Writable<W>
where
    S: Store,
    W: Debug + Send + AsyncWrite,
{
    type CID = ();

    async fn sink_leaf<L>(self, _: &mut LinkDirectoryLayer<S>, leaf: L) -> Result<Self::CID>
    where
        L: Debug + Send + AsyncRead,
    {
        use std::pin::pin;

        tokio::io::copy(&mut pin!(leaf), &mut pin!(self)).await?;
        Ok(())
    }
}
