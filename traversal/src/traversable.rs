use std::future::Future;
use tokio_stream::Stream;

use crate::TraverseBreadthFirst;

pub trait TraversableDag: Sized {
    type Error;
    type ChildStream: Stream<Item = Result<Self, Self::Error>>;
    type ChildrenFut: Future<Output = Result<Self::ChildStream, Self::Error>>;

    fn children(&self) -> Self::ChildrenFut;

    fn traverse_breadth_first(self) -> TraverseBreadthFirst<Self> {
        TraverseBreadthFirst::new(self)
    }
}
