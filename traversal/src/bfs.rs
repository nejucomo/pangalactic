use crate::ChildVisitor;
use crate::TraversableDag;
use pin_project::pin_project;
use std::{
    collections::VecDeque,
    pin::Pin,
    task::{Context, Poll},
};
use tokio_stream::Stream;

/// A stream over `D` that traverses breadth first
#[pin_project(project = TBFProjection)]
pub struct TraverseBreadthFirst<D>
where
    D: TraversableDag,
{
    queue: VecDeque<D>,
    visiting: Option<Pin<Box<ChildVisitor<D>>>>,
}

impl<D> TraverseBreadthFirst<D>
where
    D: TraversableDag,
{
    pub(crate) fn new(dag: D) -> Self {
        TraverseBreadthFirst {
            queue: VecDeque::default(),
            visiting: Some(ChildVisitor::new(dag)),
        }
    }
}

type Item<D> = Result<D, <D as TraversableDag>::Error>;
type OptItem<D> = Option<Item<D>>;
type PollOptItem<D> = Poll<OptItem<D>>;

impl<D> Stream for TraverseBreadthFirst<D>
where
    D: TraversableDag,
{
    type Item = Item<D>;

    fn poll_next(self: Pin<&mut Self>, cx: &mut Context<'_>) -> PollOptItem<D> {
        let mut proj = self.project();
        loop {
            if let Some(poi) = proj.poll_projection(cx) {
                return poi;
            }
        }
    }
}

impl<'pin, D> TBFProjection<'pin, D>
where
    D: TraversableDag,
{
    fn poll_projection(&mut self, cx: &mut Context<'_>) -> Option<PollOptItem<D>> {
        use Poll::*;

        if let Some(cv) = self.visiting.as_mut() {
            let oi = match cv.as_mut().poll_next(cx) {
                Pending => {
                    return Some(Pending);
                }
                Ready(oi) => oi,
            };

            if let Some(noderes) = oi {
                let node = match noderes {
                    Ok(n) => n,
                    error => {
                        return Some(Ready(Some(error)));
                    }
                };

                self.queue.push_back(node);
                None
            } else {
                let node = self.unwrap_visited_node();
                Some(Ready(Some(Ok(node))))
            }
        } else if let Some(node) = self.queue.pop_front() {
            *(self.visiting) = Some(ChildVisitor::new(node));
            None
        } else {
            Some(Ready(None))
        }
    }

    fn unwrap_visited_node(&mut self) -> D {
        let visiting = self.take_visiting();
        visiting.unwrap_node()
    }

    fn take_visiting(&mut self) -> ChildVisitor<D> {
        Box::into_inner(Pin::into_inner(self.visiting.take().unwrap()))
    }
}
