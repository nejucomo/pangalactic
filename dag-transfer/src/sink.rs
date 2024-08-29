use std::{
    fmt::Debug,
    future::Future,
    path::{Path, PathBuf},
};

use anyhow::{anyhow, Result};
use pangalactic_iowrappers::Writable;
use pangalactic_layer_dir::LinkDirectoryLayer;
use pangalactic_store::Store;
use tokio::{
    fs::File,
    io::{AsyncRead, AsyncWrite},
};

use crate::{
    fsutil, BranchIter, IntoSource,
    NSource::{self, Branch, Leaf},
};

pub trait Sink<L, B>: Sized + Debug + Send
where
    L: Debug + Send,
    B: Debug + Send,
{
    type CID: Send;

    fn sink(self, source: NSource<L, B>) -> impl Future<Output = Result<Self::CID>> + Send {
        async {
            match source {
                Leaf(l) => self.sink_leaf(l).await,
                Branch(b) => self.sink_branch(b).await,
            }
        }
    }

    fn sink_leaf(self, leaf: L) -> impl Future<Output = Result<Self::CID>> + Send {
        std::future::ready(Err(anyhow!("{self:?} cannot sink leaf {leaf:?}")))
    }

    fn sink_branch(self, branch: B) -> impl Future<Output = Result<Self::CID>> + Send {
        std::future::ready(Err(anyhow!("{self:?} cannot sink branch {branch:?}")))
    }
}

pub type StoreWith<'s, S, T> = (&'s LinkDirectoryLayer<S>, T);

impl<'s, 'a, S, L, B> Sink<L, B> for StoreWith<'s, S, &'a Path>
where
    S: Store,
    L: Debug + Send + AsyncRead,
    B: Debug + Send + BranchIter<L, B>,
    B::IntoSource: Send,
{
    type CID = PathBuf;

    fn sink_leaf(self, leaf: L) -> impl Future<Output = Result<Self::CID>> + Send {
        let (store, path) = self;
        <StoreWith<'s, S, PathBuf> as Sink<L, B>>::sink_leaf((store, path.to_path_buf()), leaf)
    }

    fn sink_branch(self, branch: B) -> impl Future<Output = Result<Self::CID>> + Send {
        let (store, path) = self;
        (store, path.to_path_buf()).sink_branch(branch)
    }
}

impl<'s, S, L, B> Sink<L, B> for StoreWith<'s, S, PathBuf>
where
    S: Store,
    L: Debug + Send + AsyncRead,
    B: Debug + Send + BranchIter<L, B>,
    B::IntoSource: Send,
{
    type CID = PathBuf;

    async fn sink_leaf(self, leaf: L) -> Result<Self::CID> {
        let (_, path) = self;
        let f = fsutil::create_file(&path).await?;
        f.sink_leaf(leaf).await?;
        Ok(path)
    }

    async fn sink_branch(self, mut branch: B) -> Result<Self::CID> {
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

impl<L> Sink<L, ()> for File
where
    L: Debug + Send + AsyncRead,
{
    type CID = ();

    fn sink_leaf(self, leaf: L) -> impl Future<Output = Result<Self::CID>> + Send {
        Writable(self).sink_leaf(leaf)
    }
}

impl<W, L> Sink<L, ()> for Writable<W>
where
    W: Debug + Send + AsyncWrite,
    L: Debug + Send + AsyncRead,
{
    type CID = ();

    async fn sink_leaf(self, leaf: L) -> Result<Self::CID> {
        use std::pin::pin;

        tokio::io::copy(&mut pin!(leaf), &mut pin!(self)).await?;
        Ok(())
    }
}
