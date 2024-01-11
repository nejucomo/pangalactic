use std::{
    future::Future,
    pin::Pin,
    task::{Context, Poll},
};
use tokio_stream::Stream;

#[derive(Debug)]
pub(super) enum Inner<F, S, X>
where
    F: Future<Output = S>,
    S: Stream<Item = X>,
{
    /// Pending the stream output of the future:
    F(Pin<Box<F>>),
    /// Stream is active:
    S(Pin<Box<S>>),
}

impl<F, S, X> From<F> for Inner<F, S, X>
where
    F: Future<Output = S>,
    S: Stream<Item = X>,
{
    fn from(init: F) -> Self {
        Inner::F(Box::pin(init))
    }
}

impl<F, S, X> Inner<F, S, X>
where
    F: Future<Output = S>,
    S: Stream<Item = X>,
{
    pub(super) fn poll_inner(&mut self, cx: &mut Context<'_>) -> Poll<Option<X>> {
        use std::task::ready;
        use Inner::*;

        match self {
            F(fut) => {
                let stream = ready!(fut.as_mut().poll(cx));
                *self = S(Box::pin(stream));
                self.poll_inner(cx)
            }
            S(stream) => stream.as_mut().poll_next(cx),
        }
    }
}
