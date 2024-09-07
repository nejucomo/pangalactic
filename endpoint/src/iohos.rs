use std::future::Future;

use anyhow::Result;
use pin_project::pin_project;
use tokio::io::AsyncRead;

use crate::hos::Hos;

use self::Iohos::*;

#[pin_project(project = IohosProjection)]
#[derive(Clone, Debug, Eq, PartialEq)]
pub(crate) enum Iohos<IO, H, S> {
    MkStdio(#[pin] IO),
    MkHos(#[pin] Hos<H, S>),
}

impl<IO, H, S> Iohos<IO, H, S> {
    pub fn as_ref(&self) -> Iohos<&IO, &H, &S> {
        match self {
            MkStdio(ref io) => MkStdio(io),
            MkHos(hos) => MkHos(hos.as_ref()),
        }
    }

    pub fn map_io<F, IO2>(self, f: F) -> Iohos<IO2, H, S>
    where
        F: FnOnce(IO) -> IO2,
    {
        match self {
            MkStdio(io) => MkStdio(f(io)),
            MkHos(hos) => MkHos(hos),
        }
    }

    pub fn map_host<F, H2>(self, f: F) -> Iohos<IO, H2, S>
    where
        F: FnOnce(H) -> H2,
    {
        self.map_host_or_store(|hos| hos.map_host(f))
    }

    pub fn map_store<F, S2>(self, f: F) -> Iohos<IO, H, S2>
    where
        F: FnOnce(S) -> S2,
    {
        self.map_host_or_store(|hos| hos.map_store(f))
    }

    pub fn map_host_or_store<F, H2, S2>(self, f: F) -> Iohos<IO, H2, S2>
    where
        F: FnOnce(Hos<H, S>) -> Hos<H2, S2>,
    {
        match self {
            MkStdio(io) => MkStdio(io),
            MkHos(hos) => MkHos(f(hos)),
        }
    }

    pub fn map_into<FIO, FH, FS, T>(self, io_into: FIO, host_into: FH, store_into: FS) -> T
    where
        FIO: FnOnce(IO) -> T,
        FH: FnOnce(H) -> T,
        FS: FnOnce(S) -> T,
    {
        self.map_into_with_host_or_store(io_into, |hos| hos.map_into(host_into, store_into))
    }

    pub fn map_into_with_host_or_store<FIO, FHOS, T>(self, io_into: FIO, hos_into: FHOS) -> T
    where
        FIO: FnOnce(IO) -> T,
        FHOS: FnOnce(Hos<H, S>) -> T,
    {
        match self {
            MkStdio(io) => io_into(io),
            MkHos(hos) => hos_into(hos),
        }
    }

    pub async fn await_futures(
        self,
    ) -> Iohos<<IO as Future>::Output, <H as Future>::Output, <S as Future>::Output>
    where
        IO: Future,
        H: Future,
        S: Future,
    {
        match self {
            MkStdio(io) => MkStdio(io.await),
            MkHos(hos) => MkHos(hos.await_futures().await),
        }
    }
}

impl<T> Iohos<T, T, T> {
    pub fn distill(self) -> T {
        match self {
            MkStdio(t) => t,
            MkHos(hos) => hos.distill(),
        }
    }
}

impl<IO, H, S, E> Iohos<Result<IO, E>, Result<H, E>, Result<S, E>> {
    pub fn transpose(self) -> Result<Iohos<IO, H, S>, E> {
        match self {
            MkStdio(io) => io.map(MkStdio),
            MkHos(hos) => hos.transpose().map(MkHos),
        }
    }
}

impl<IO, H, S> AsyncRead for Iohos<IO, H, S>
where
    IO: AsyncRead,
    Hos<H, S>: AsyncRead,
{
    fn poll_read(
        self: std::pin::Pin<&mut Self>,
        cx: &mut std::task::Context<'_>,
        buf: &mut tokio::io::ReadBuf<'_>,
    ) -> std::task::Poll<std::io::Result<()>> {
        use IohosProjection::*;

        match self.project() {
            MkStdio(io) => io.poll_read(cx, buf),
            MkHos(hos) => hos.poll_read(cx, buf),
        }
    }
}

impl<IO, H, S> From<Hos<H, S>> for Iohos<IO, H, S> {
    fn from(hos: Hos<H, S>) -> Self {
        MkHos(hos)
    }
}
