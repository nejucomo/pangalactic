use std::path::PathBuf;

use pangalactic_dag_transfer::IntoSource;
use pangalactic_linkpath::LinkPath;
use pangalactic_store::Store;
use pin_project::pin_project;
use tokio::io::{AsyncRead, Stdin};

use crate::iohos::Iohos;

#[pin_project]
#[derive(Debug)]
pub struct SourceEndpointLeaf<S>(
    #[pin]
    pub(crate)  Iohos<
        <Stdin as IntoSource<S>>::Leaf,
        <PathBuf as IntoSource<S>>::Leaf,
        <LinkPath<S::CID> as IntoSource<S>>::Leaf,
    >,
)
where
    S: Store;

impl<S> AsyncRead for SourceEndpointLeaf<S>
where
    S: Store,
{
    fn poll_read(
        self: std::pin::Pin<&mut Self>,
        cx: &mut std::task::Context<'_>,
        buf: &mut tokio::io::ReadBuf<'_>,
    ) -> std::task::Poll<std::io::Result<()>> {
        self.project().0.poll_read(cx, buf)
    }
}
