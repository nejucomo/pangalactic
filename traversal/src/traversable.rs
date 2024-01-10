use std::future::Future;
use tokio_stream::Stream;

pub trait TraversableDag: Sized {
    type Error;
    type ChildStream: Stream<Item = Result<Self, Self::Error>>;
    type ChildrenFut: Future<Output = Result<Self::ChildStream, Self::Error>>;

    fn children(&self) -> Self::ChildrenFut;
}
