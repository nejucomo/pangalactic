use std::{
    future::Future,
    pin::Pin,
    task::{Context, Poll},
};
use tokio_stream::Stream;

#[derive(Debug)]
pub(super) enum Inner<F, S, X, E>
where
    F: Future<Output = Result<S, E>>,
    S: Stream<Item = Result<X, E>>,
{
    /// Pending the stream output of the future:
    F(Pin<Box<F>>),
    /// Stream is active:
    S(Pin<Box<S>>),
}

impl<F, S, X, E> From<F> for Inner<F, S, X, E>
where
    F: Future<Output = Result<S, E>>,
    S: Stream<Item = Result<X, E>>,
{
    fn from(init: F) -> Self {
        Inner::F(Box::pin(init))
    }
}

impl<F, S, X, E> Inner<F, S, X, E>
where
    F: Future<Output = Result<S, E>>,
    S: Stream<Item = Result<X, E>>,
{
    pub(super) fn poll_inner(&mut self, cx: &mut Context<'_>) -> Poll<Option<Result<X, E>>> {
        use std::task::ready;
        use Inner::*;

        match self {
            F(fut) => {
                let stream = ready!(fut.as_mut().poll(cx))?;
                *self = S(Box::pin(stream));
                self.poll_inner(cx)
            }
            S(stream) => stream.as_mut().poll_next(cx),
        }
    }
}
