use crate::AsyncTryIterator;

pub trait IntoAsyncTryIterator {
    type Item;
    type ATI: AsyncTryIterator<Item = Self::Item>;

    fn into_async_try_iter(self) -> Self::ATI;
}

impl<T> IntoAsyncTryIterator for T
where
    T: AsyncTryIterator,
{
    type Item = T::Item;
    type ATI = Self;

    fn into_async_try_iter(self) -> Self::ATI {
        self
    }
}
