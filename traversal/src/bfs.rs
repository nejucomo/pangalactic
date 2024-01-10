use crate::TraversableDag;
use pin_project::pin_project;
use std::{
    collections::VecDeque,
    future::Future,
    pin::Pin,
    task::{Context, Poll},
};
use tokio_stream::Stream;

#[pin_project(project = TBFProjection)]
pub struct TraverseBreadthFirst<D>
where
    D: TraversableDag,
{
    queue: VecDeque<D>,
    pending: PendingState<D>,
}

enum PendingState<D>
where
    D: TraversableDag,
{
    PendingNew,
    CFuture(Pin<Box<D::ChildrenFut>>),
    CStream(Pin<Box<D::ChildStream>>),
}
use PendingState::*;

impl<D> TraverseBreadthFirst<D>
where
    D: TraversableDag,
{
    pub(crate) fn new(dag: D) -> Self {
        TraverseBreadthFirst {
            queue: VecDeque::from([dag]),
            pending: PendingNew,
        }
    }
}

type StreamItem<D> = Result<D, <D as TraversableDag>::Error>;
type StreamReady<D> = Option<StreamItem<D>>;

impl<'pin, D> TBFProjection<'pin, D>
where
    D: TraversableDag,
{
    /// Handle all of `poll_next` from this projection:
    fn poll_projection(mut self, cx: &mut Context<'_>) -> Poll<StreamReady<D>> {
        loop {
            if let Some(poitem) = self.poll_inner(cx) {
                return poitem;
            }
        }
    }

    /// Run inner state transitions via this projection.
    ///
    /// A return value of `None` indicates `poll_next` needs to recurse to ensure a newly established async value registers with `cx`. Otherwise the `Some` result can be returned directly in the outer `poll_next`.
    fn poll_inner(&mut self, cx: &mut Context<'_>) -> Option<Poll<StreamReady<D>>> {
        use Poll::*;

        match self.pending {
            PendingNew => {
                if let Some(node) = self.queue.front() {
                    *(self.pending) = CFuture(Box::pin(node.children()));
                    None
                } else {
                    Some(Ready(None))
                }
            }
            CFuture(fut) => match fut.as_mut().poll(cx) {
                Ready(streamres) => match streamres {
                    Ok(stream) => {
                        *(self.pending) = CStream(Box::pin(stream));
                        None
                    }
                    Err(e) => Some(Ready(Some(Err(e)))),
                },
                Pending => Some(Pending),
            },
            CStream(stream) => match stream.as_mut().poll_next(cx) {
                Ready(Some(Ok(node))) => {
                    self.queue.push_back(node);
                    None
                }
                Ready(Some(Err(e))) => Some(Ready(Some(Err(e)))),
                Ready(None) => {
                    // BUG: factor out this unwrap with better type definitions:
                    let node = self.queue.pop_front().unwrap();
                    *(self.pending) = PendingNew;
                    Some(Ready(Some(Ok(node))))
                }
                Pending => Some(Pending),
            },
        }
    }
}

impl<D> Stream for TraverseBreadthFirst<D>
where
    D: TraversableDag,
{
    type Item = StreamItem<D>;

    fn poll_next(self: Pin<&mut Self>, cx: &mut Context<'_>) -> Poll<StreamReady<D>> {
        self.project().poll_projection(cx)
    }
}
