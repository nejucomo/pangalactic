use crate::TraversableDag;
use pangalactic_init_stream::FallibleInitStream;
use pin_project::pin_project;
use std::{
    pin::Pin,
    task::{Context, Poll},
};
use tokio_stream::Stream;

type ChildStream<D> = FallibleInitStream<
    <D as TraversableDag>::ChildrenFut,
    <D as TraversableDag>::ChildStream,
    D,
    <D as TraversableDag>::Error,
>;

#[pin_project]
pub(super) struct ChildVisitor<D>
where
    D: TraversableDag,
{
    node: D,
    children: Pin<Box<ChildStream<D>>>,
}

impl<D> ChildVisitor<D>
where
    D: TraversableDag,
{
    pub(super) fn new(node: D) -> Pin<Box<Self>> {
        let children = node.children();
        Box::pin(ChildVisitor {
            node,
            children: Box::pin(FallibleInitStream::from(children)),
        })
    }

    pub(super) fn unwrap_node(self) -> D {
        self.node
    }
}

impl<D> Stream for ChildVisitor<D>
where
    D: TraversableDag,
{
    type Item = Result<D, D::Error>;

    fn poll_next(self: Pin<&mut Self>, cx: &mut Context<'_>) -> Poll<Option<Self::Item>> {
        self.project().children.as_mut().poll_next(cx)
    }
}
