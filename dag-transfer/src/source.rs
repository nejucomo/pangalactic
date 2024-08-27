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
pub struct BranchSource<I, T>(pub I)
where
    I: IntoAsyncTryIterator<Item = (Name, T)> + Send,
    T: IntoSource;

#[derive(Debug)]
pub enum LeafOrBranchSource<R, I, T>
where
    R: AsyncRead + Send,
    I: IntoAsyncTryIterator<Item = (Name, T)> + Send,
    T: IntoSource,
{
    Leaf(R),
    Branch(I),
}

impl<R, I, T> From<LeafSource<R>> for LeafOrBranchSource<R, I, T>
where
    R: AsyncRead + Send,
    I: IntoAsyncTryIterator<Item = (Name, T)> + Send,
    T: IntoSource,
{
    fn from(leaf: LeafSource<R>) -> Self {
        LeafOrBranchSource::Leaf(leaf.0)
    }
}

impl<R, I, T> From<BranchSource<I, T>> for LeafOrBranchSource<R, I, T>
where
    R: AsyncRead + Send,
    I: IntoAsyncTryIterator<Item = (Name, T)> + Send,
    T: IntoSource,
{
    fn from(branch: BranchSource<I, T>) -> Self {
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

        impl<I, T> Source for BranchSource<I, T>
        where
            I: IntoAsyncTryIterator<Item = (Name, T)> + Send,
            T: IntoSource,
        {
        }

        impl<I, T> self::sealed::Sealed for BranchSource<I, T>
        where
            I: IntoAsyncTryIterator<Item = (Name, T)> + Send,
            T: IntoSource,
        {
        }

        impl<I, T> IntoSource for BranchSource<I, T>
        where
            I: IntoAsyncTryIterator<Item = (Name, T)> + Send,
            T: IntoSource,
        {
            type Source = Self;

            fn into_source(self) -> impl Future<Output = Result<Self::Source>> + Send {
                std::future::ready(Ok(self))
            }
        }
    }

    mod leaf_or_branch_impls {
        use super::*;

        impl<R, I, T> Source for LeafOrBranchSource<R, I, T>
        where
            R: AsyncRead + Send,
            I: IntoAsyncTryIterator<Item = (Name, T)> + Send,
            T: IntoSource,
        {
        }

        impl<R, I, T> self::sealed::Sealed for LeafOrBranchSource<R, I, T>
        where
            R: AsyncRead + Send,
            I: IntoAsyncTryIterator<Item = (Name, T)> + Send,
            T: IntoSource,
        {
        }

        impl<R, I, T> IntoSource for LeafOrBranchSource<R, I, T>
        where
            R: AsyncRead + Send,
            I: IntoAsyncTryIterator<Item = (Name, T)> + Send,
            T: IntoSource,
        {
            type Source = Self;

            fn into_source(self) -> impl Future<Output = Result<Self::Source>> + Send {
                std::future::ready(Ok(self))
            }
        }
    }
}
