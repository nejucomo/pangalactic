//! Wrap a [Future] that produces a [Stream] into a [Stream] that envelops the "initialization" future.

#![deny(unsafe_code, missing_docs)]

mod fstream;
mod stream;

use std::future::Future;
use tokio_stream::Stream;

pub use self::fstream::FallibleInitStream;
pub use self::stream::InitStream;

/// Given a [Future], `init` which produces a [Stream] over `X`, construct a [Stream] over `X` that resolves `init` internally
pub fn from<F, S, X>(init: F) -> InitStream<F, S, X>
where
    F: Future<Output = S>,
    S: Stream<Item = X>,
{
    InitStream::from(init)
}

/// Given a [Future], `init` which produces [Result] with [Stream] over `X`, construct a [Stream] over `Result<X, E>` that resolves `init` internally
///
/// An [Err] from `init` propagates to the first [Stream] item as the same error.
pub fn from_fallible<F, S, X, E>(init: F) -> FallibleInitStream<F, S, X, E>
where
    F: Future<Output = Result<S, E>>,
    S: Stream<Item = Result<X, E>>,
{
    FallibleInitStream::from(init)
}
