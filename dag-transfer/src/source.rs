use std::future::Future;

use anyhow::Result;
use tokio::io::AsyncRead;

use crate::{BranchIter, IntoSource};

mod sealed {
    pub trait Sealed {}
}

pub trait Source: self::sealed::Sealed {}

#[derive(Debug)]
pub struct LeafSource<L>(pub L)
where
    L: AsyncRead + Send;

#[derive(Debug)]
pub struct BranchSource<B>(pub B)
where
    B: BranchIter;

#[derive(Debug)]
pub enum LeafOrBranchSource<L, B>
where
    L: AsyncRead + Send,
    B: BranchIter,
{
    Leaf(L),
    Branch(B),
}

impl<L, B> From<LeafSource<L>> for LeafOrBranchSource<L, B>
where
    L: AsyncRead + Send,
    B: BranchIter,
{
    fn from(leaf: LeafSource<L>) -> Self {
        LeafOrBranchSource::Leaf(leaf.0)
    }
}

impl<L, B> From<BranchSource<B>> for LeafOrBranchSource<L, B>
where
    L: AsyncRead + Send,
    B: BranchIter,
{
    fn from(branch: BranchSource<B>) -> Self {
        LeafOrBranchSource::Branch(branch.0)
    }
}

mod impls {
    //! FIXME: I cannot determine a way to compress the following boilerplate to the meaningful bits:

    use super::*;

    mod leaf_impls {
        use super::*;

        impl<L> Source for LeafSource<L> where L: AsyncRead + Send {}
        impl<L> self::sealed::Sealed for LeafSource<L> where L: AsyncRead + Send {}

        impl<L> IntoSource for LeafSource<L>
        where
            L: AsyncRead + Send,
        {
            type Source = Self;

            fn into_source(self) -> impl Future<Output = Result<Self::Source>> + Send {
                std::future::ready(Ok(self))
            }
        }
    }

    mod branch_impls {
        use pangalactic_name::Name;

        use super::*;

        impl<B> Source for BranchSource<B> where B: BranchIter {}

        impl<B> self::sealed::Sealed for BranchSource<B> where B: BranchIter {}

        impl<B> IntoSource for BranchSource<B>
        where
            B: BranchIter,
        {
            type Source = Self;

            fn into_source(self) -> impl Future<Output = Result<Self::Source>> + Send {
                std::future::ready(Ok(self))
            }
        }

        impl<B> BranchIter for BranchSource<B>
        where
            B: BranchIter,
        {
            type IntoSource = B::IntoSource;

            fn next_branch_entry(
                &mut self,
            ) -> impl Future<Output = Result<Option<(Name, B::IntoSource)>>> + Send {
                self.0.next_branch_entry()
            }
        }
    }

    mod leaf_or_branch_impls {
        use super::*;

        impl<L, B> Source for LeafOrBranchSource<L, B>
        where
            L: AsyncRead + Send,
            B: BranchIter,
        {
        }

        impl<L, B> self::sealed::Sealed for LeafOrBranchSource<L, B>
        where
            L: AsyncRead + Send,
            B: BranchIter,
        {
        }

        impl<L, B> IntoSource for LeafOrBranchSource<L, B>
        where
            L: AsyncRead + Send,
            B: BranchIter,
        {
            type Source = Self;

            fn into_source(self) -> impl Future<Output = Result<Self::Source>> + Send {
                std::future::ready(Ok(self))
            }
        }
    }
}
