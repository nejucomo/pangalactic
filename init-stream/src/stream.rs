mod inner;

use self::inner::Inner;
use pin_project::pin_project;
use std::{
    future::Future,
    pin::Pin,
    task::{Context, Poll},
};
use tokio_stream::Stream;

/// A [Stream] over `X` which encapsulates an initialization [Future] that produces an underlying stream over `X`
#[pin_project]
#[derive(Debug)]
pub struct InitStream<F, S, X>(Inner<F, S, X>)
where
    F: Future<Output = S>,
    S: Stream<Item = X>;

impl<F, S, X> From<F> for InitStream<F, S, X>
where
    F: Future<Output = S>,
    S: Stream<Item = X>,
{
    fn from(init: F) -> Self {
        InitStream(Inner::from(init))
    }
}

impl<F, S, X> Stream for InitStream<F, S, X>
where
    F: Future<Output = S>,
    S: Stream<Item = X>,
{
    type Item = X;

    fn poll_next(self: Pin<&mut Self>, cx: &mut Context<'_>) -> Poll<Option<Self::Item>> {
        self.project().0.poll_inner(cx)
    }
}
