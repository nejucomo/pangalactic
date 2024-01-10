mod inner;

use self::inner::Inner;
use pin_project::pin_project;
use std::{
    future::Future,
    pin::Pin,
    task::{Context, Poll},
};
use tokio_stream::Stream;

/// A [Stream] over `Result<X, E>` which encapsulates an initialization [Future] that produces `Result<S, E>` where `S` is the underlying stream.
#[pin_project]
#[derive(Debug)]
pub struct FallibleInitStream<F, S, X, E>(Inner<F, S, X, E>)
where
    F: Future<Output = Result<S, E>>,
    S: Stream<Item = Result<X, E>>;

impl<F, S, X, E> From<F> for FallibleInitStream<F, S, X, E>
where
    F: Future<Output = Result<S, E>>,
    S: Stream<Item = Result<X, E>>,
{
    fn from(init: F) -> Self {
        FallibleInitStream(Inner::from(init))
    }
}

impl<F, S, X, E> Stream for FallibleInitStream<F, S, X, E>
where
    F: Future<Output = Result<S, E>>,
    S: Stream<Item = Result<X, E>>,
{
    type Item = Result<X, E>;

    fn poll_next(self: Pin<&mut Self>, cx: &mut Context<'_>) -> Poll<Option<Self::Item>> {
        self.project().0.poll_inner(cx)
    }
}
