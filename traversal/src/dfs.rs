use crate::ChildVisitor;
use crate::TraversableDag;
use pin_project::pin_project;
use std::{
    pin::Pin,
    task::{Context, Poll},
};
use tokio_stream::Stream;

/// A stream over `D` that traverses breadth first
#[pin_project]
pub struct TraverseDepthFirst<D>
where
    D: TraversableDag,
{
    stack: Vec<Pin<Box<ChildVisitor<D>>>>,
}

impl<D> TraverseDepthFirst<D>
where
    D: TraversableDag,
{
    pub(crate) fn new(dag: D) -> Self {
        TraverseDepthFirst {
            stack: vec![ChildVisitor::new(dag)],
        }
    }
}

impl<D> Stream for TraverseDepthFirst<D>
where
    D: TraversableDag,
{
    type Item = Result<D, D::Error>;

    fn poll_next(self: Pin<&mut Self>, cx: &mut Context<'_>) -> Poll<Option<Self::Item>> {
        use std::task::ready;
        use Poll::*;

        let proj = self.project();
        loop {
            if let Some(top) = proj.stack.last_mut() {
                if let Some(res) = ready!(top.as_mut().poll_next(cx)) {
                    let child = res?;
                    proj.stack.push(ChildVisitor::new(child));
                } else {
                    let cv = proj.stack.pop().unwrap();
                    let node = Box::into_inner(Pin::into_inner(cv)).unwrap_node();
                    return Ready(Some(Ok(node)));
                }
            } else {
                return Ready(None);
            }
        }
    }
}
