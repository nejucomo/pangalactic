use std::future::Future;

use anyhow::Result;

use crate::AsyncTryIterator;

#[derive(Debug)]
pub struct TryMapAsync<I, F, Fut, T>
where
    I: Send + AsyncTryIterator,
    <I as AsyncTryIterator>::Item: Send,
    F: Send + Sync + Fn(I::Item) -> Fut,
    Fut: Send + Future<Output = Result<T>>,
{
    inner: I,
    f: F,
}

impl<I, F, Fut, T> TryMapAsync<I, F, Fut, T>
where
    I: Send + AsyncTryIterator,
    <I as AsyncTryIterator>::Item: Send,
    F: Send + Sync + Fn(I::Item) -> Fut,
    Fut: Send + Future<Output = Result<T>>,
{
    pub(crate) fn new(inner: I, f: F) -> Self {
        Self { inner, f }
    }
}

impl<I, F, Fut, T> AsyncTryIterator for TryMapAsync<I, F, Fut, T>
where
    I: Send + AsyncTryIterator,
    <I as AsyncTryIterator>::Item: Send,
    F: Send + Sync + Fn(I::Item) -> Fut,
    Fut: Send + Future<Output = Result<T>>,
{
    type Item = T;

    fn try_next_async(&mut self) -> impl Future<Output = Result<Option<Self::Item>>> + Send {
        async {
            if let Some(inner) = self.inner.try_next_async().await? {
                let outer = (self.f)(inner).await?;
                Ok(Some(outer))
            } else {
                Ok(None)
            }
        }
    }
}
