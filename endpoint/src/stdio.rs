use std::{
    fmt,
    future::{ready, Future},
    str::FromStr,
};

use anyhow::{anyhow, Result};
use futures::FutureExt;
use pangalactic_dag_transfer::{BranchIter, Destination, IntoSource, LeafDestination, Source};
use pangalactic_layer_dir::LinkDirectoryLayer;
use pangalactic_store::Store;
use tokio::{fs::ReadDir, io::AsyncRead};

#[derive(Copy, Clone, Debug, Eq, PartialEq)]
pub struct Stdio;

impl<S> IntoSource<S> for Stdio
where
    S: Store,
{
    type Leaf = Self;
    type Branch = ReadDir;

    fn into_source(
        self,
        _: &LinkDirectoryLayer<S>,
    ) -> impl Future<Output = Result<Source<Self::Leaf, Self::Branch>>> + Send {
        ready(Ok(Source::Leaf(self)))
    }
}

impl<S> Destination<S> for Stdio
where
    S: Store,
{
    fn sink_branch<B>(
        self,
        _store: &mut LinkDirectoryLayer<S>,
        _branch: B,
    ) -> impl Future<Output = Result<Self::CID>> + Send
    where
        B: fmt::Debug + Send + BranchIter<S>,
    {
        ready(Err(anyhow!(
            "cannot transfer a directory-like structure to stdout"
        )))
    }
}

impl<S> LeafDestination<S> for Stdio
where
    S: Store,
{
    type CID = Self;

    fn sink_leaf<L>(
        self,
        store: &mut LinkDirectoryLayer<S>,
        leaf: L,
    ) -> impl Future<Output = Result<Self::CID>> + Send
    where
        L: fmt::Debug + Send + AsyncRead,
    {
        tokio::io::stdout()
            .sink_leaf(store, leaf)
            .map(|res| res.map(|()| Stdio))
    }
}

impl AsyncRead for Stdio {
    fn poll_read(
        self: std::pin::Pin<&mut Self>,
        cx: &mut std::task::Context<'_>,
        buf: &mut tokio::io::ReadBuf<'_>,
    ) -> std::task::Poll<std::io::Result<()>> {
        std::pin::pin!(tokio::io::stdin()).poll_read(cx, buf)
    }
}

impl fmt::Display for Stdio {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        '-'.fmt(f)
    }
}

impl FromStr for Stdio {
    type Err = anyhow::Error;

    fn from_str(s: &str) -> Result<Self> {
        if s == "-" {
            Ok(Stdio)
        } else {
            anyhow::bail!(r#"expected "-", found {s:?}"#)
        }
    }
}
