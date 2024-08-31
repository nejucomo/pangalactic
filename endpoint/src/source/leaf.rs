use std::path::PathBuf;

use pangalactic_dag_transfer::IntoSource;
use pangalactic_linkpath::LinkPath;
use pangalactic_store::Store;
use pin_project::pin_project;
use tokio::io::{AsyncRead, Stdin};

#[pin_project(project = SELProjection)]
#[derive(Debug)]
pub enum SourceEndpointLeaf<S>
where
    S: Store,
{
    Stdin(#[pin] <Stdin as IntoSource<S>>::Leaf),
    Host(#[pin] <PathBuf as IntoSource<S>>::Leaf),
    Store(#[pin] <LinkPath<S::CID> as IntoSource<S>>::Leaf),
}

impl<S> AsyncRead for SourceEndpointLeaf<S>
where
    S: Store,
{
    fn poll_read(
        self: std::pin::Pin<&mut Self>,
        cx: &mut std::task::Context<'_>,
        buf: &mut tokio::io::ReadBuf<'_>,
    ) -> std::task::Poll<std::io::Result<()>> {
        use SELProjection::*;

        match self.project() {
            Stdin(x) => x.poll_read(cx, buf),
            Host(x) => x.poll_read(cx, buf),
            Store(x) => x.poll_read(cx, buf),
        }
    }
}
