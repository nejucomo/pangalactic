use std::future::Future;
use tokio_stream::Stream;

use crate::TraverseBreadthFirst;

/// A Directed Acyclic Graph node which can be traversed fallibly and asynchronously
pub trait TraversableDag: Sized {
    /// The error types for traversal
    type Error;

    /// A fallible stream over the direct children of this node
    type ChildStream: Stream<Item = Result<Self, Self::Error>>;

    /// A fallible future for [Self::ChildStream]
    type ChildrenFut: Future<Output = Result<Self::ChildStream, Self::Error>>;

    /// Begin traversing direct children of this node
    fn children(&self) -> Self::ChildrenFut;

    /// Traverse the entire sub-DAG from this node, breadth first
    fn traverse_breadth_first(self) -> TraverseBreadthFirst<Self> {
        TraverseBreadthFirst::new(self)
    }
}
