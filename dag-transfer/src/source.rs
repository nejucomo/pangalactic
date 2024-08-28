use std::future::Future;

use anyhow::Result;
use pangalactic_asynctryiter::IntoAsyncTryIterator;
use pangalactic_name::Name;
use tokio::io::AsyncRead;

use crate::IntoSource;

mod sealed {
    pub trait Sealed {}
}

pub trait Source: self::sealed::Sealed {}

#[derive(Debug)]
pub struct LeafSource<R>(pub R)
where
    R: AsyncRead + Send;

#[derive(Debug)]
pub struct BranchSource<R, I>(pub I)
where
    R: AsyncRead + Send,
    I: IntoAsyncTryIterator<Item = (Name, LeafOrBranchSource<R, I>)> + Send;

#[derive(Debug)]
pub enum LeafOrBranchSource<R, I>
where
    R: AsyncRead + Send,
    I: IntoAsyncTryIterator<Item = (Name, LeafOrBranchSource<R, I>)> + Send,
{
    Leaf(R),
    Branch(I),
}

impl<R, I> From<LeafSource<R>> for LeafOrBranchSource<R, I>
where
    R: AsyncRead + Send,
    I: IntoAsyncTryIterator<Item = (Name, Self)> + Send,
{
    fn from(leaf: LeafSource<R>) -> Self {
        LeafOrBranchSource::Leaf(leaf.0)
    }
}

impl<R, I> From<BranchSource<R, I>> for LeafOrBranchSource<R, I>
where
    R: AsyncRead + Send,
    I: IntoAsyncTryIterator<Item = (Name, Self)> + Send,
{
    fn from(branch: BranchSource<R, I>) -> Self {
        LeafOrBranchSource::Branch(branch.0)
    }
}

mod impls {
    //! FIXME: I cannot determine a way to compress the following boilerplate to the meaningful bits:

    use super::*;

    mod leaf_impls {
        use super::*;

        impl<R> Source for LeafSource<R> where R: AsyncRead + Send {}
        impl<R> self::sealed::Sealed for LeafSource<R> where R: AsyncRead + Send {}

        impl<R> IntoSource for LeafSource<R>
        where
            R: AsyncRead + Send,
        {
            type Source = Self;

            fn into_source(self) -> impl Future<Output = Result<Self::Source>> + Send {
                std::future::ready(Ok(self))
            }
        }
    }

    mod branch_impls {
        use super::*;

        impl<R, I> IntoAsyncTryIterator for BranchSource<R, I>
        where
            R: AsyncRead + Send,
            I: IntoAsyncTryIterator<Item = (Name, LeafOrBranchSource<R, I>)> + Send,
        {
            type Item = (Name, LeafOrBranchSource<R, I>);
            type ATI = I::ATI;

            fn into_async_try_iter(self) -> Self::ATI {
                self.0.into_async_try_iter()
            }
        }

        impl<R, I> Source for BranchSource<R, I>
        where
            R: AsyncRead + Send,
            I: IntoAsyncTryIterator<Item = (Name, LeafOrBranchSource<R, I>)> + Send,
        {
        }

        impl<R, I> self::sealed::Sealed for BranchSource<R, I>
        where
            R: AsyncRead + Send,
            I: IntoAsyncTryIterator<Item = (Name, LeafOrBranchSource<R, I>)> + Send,
        {
        }

        impl<R, I> IntoSource for BranchSource<R, I>
        where
            R: AsyncRead + Send,
            I: IntoAsyncTryIterator<Item = (Name, LeafOrBranchSource<R, I>)> + Send,
        {
            type Source = Self;

            fn into_source(self) -> impl Future<Output = Result<Self::Source>> + Send {
                std::future::ready(Ok(self))
            }
        }
    }

    mod leaf_or_branch_impls {
        use super::*;

        impl<R, I> Source for LeafOrBranchSource<R, I>
        where
            R: AsyncRead + Send,
            I: IntoAsyncTryIterator<Item = (Name, LeafOrBranchSource<R, I>)> + Send,
        {
        }

        impl<R, I> self::sealed::Sealed for LeafOrBranchSource<R, I>
        where
            R: AsyncRead + Send,
            I: IntoAsyncTryIterator<Item = (Name, LeafOrBranchSource<R, I>)> + Send,
        {
        }

        impl<R, I> IntoSource for LeafOrBranchSource<R, I>
        where
            R: AsyncRead + Send,
            I: IntoAsyncTryIterator<Item = (Name, LeafOrBranchSource<R, I>)> + Send,
        {
            type Source = Self;

            fn into_source(self) -> impl Future<Output = Result<Self::Source>> + Send {
                std::future::ready(Ok(self))
            }
        }
    }
}
