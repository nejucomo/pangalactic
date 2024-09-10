use std::{fmt, future::Future, str::FromStr};

use anyhow::Result;
use pin_project::pin_project;
use tokio::io::AsyncRead;

use crate::hostorstore::HostOrStore;

use self::Endpoint::*;

#[pin_project(project = EndpointProjection)]
#[derive(Clone, Debug, Eq, PartialEq)]
pub enum Endpoint<IO, H, S> {
    MkStdio(#[pin] IO),
    MkHos(#[pin] HostOrStore<H, S>),
}

impl<IO, H, S> Endpoint<IO, H, S> {
    pub fn as_ref(&self) -> Endpoint<&IO, &H, &S> {
        match self {
            MkStdio(ref io) => MkStdio(io),
            MkHos(hos) => MkHos(hos.as_ref()),
        }
    }

    pub fn map_io<F, IO2>(self, f: F) -> Endpoint<IO2, H, S>
    where
        F: FnOnce(IO) -> IO2,
    {
        match self {
            MkStdio(io) => MkStdio(f(io)),
            MkHos(hos) => MkHos(hos),
        }
    }

    pub fn map_host<F, H2>(self, f: F) -> Endpoint<IO, H2, S>
    where
        F: FnOnce(H) -> H2,
    {
        self.map_host_or_store(|hos| hos.map_host(f))
    }

    pub fn map_store<F, S2>(self, f: F) -> Endpoint<IO, H, S2>
    where
        F: FnOnce(S) -> S2,
    {
        self.map_host_or_store(|hos| hos.map_store(f))
    }

    fn map_host_or_store<F, H2, S2>(self, f: F) -> Endpoint<IO, H2, S2>
    where
        F: FnOnce(HostOrStore<H, S>) -> HostOrStore<H2, S2>,
    {
        match self {
            MkStdio(io) => MkStdio(io),
            MkHos(hos) => MkHos(f(hos)),
        }
    }

    pub fn project_into<FIO, FH, FS, T>(self, io_into: FIO, host_into: FH, store_into: FS) -> T
    where
        FIO: FnOnce(IO) -> T,
        FH: FnOnce(H) -> T,
        FS: FnOnce(S) -> T,
    {
        self.project_into_with(
            (),
            |io, ()| io_into(io),
            |h, ()| host_into(h),
            |s, ()| store_into(s),
        )
    }

    pub fn map_any_with<A, FIO, RIO, FH, RH, FS, RS>(
        self,
        arg: A,
        map_io: FIO,
        map_host: FH,
        map_store: FS,
    ) -> Endpoint<RIO, RH, RS>
    where
        FIO: FnOnce(IO, A) -> RIO,
        FH: FnOnce(H, A) -> RH,
        FS: FnOnce(S, A) -> RS,
    {
        match self {
            MkStdio(io) => MkStdio(map_io(io, arg)),
            MkHos(hos) => MkHos(hos.map_either_with(arg, map_host, map_store)),
        }
    }

    pub fn project_into_with<A, FIO, FH, FS, R>(
        self,
        arg: A,
        io_into: FIO,
        host_into: FH,
        store_into: FS,
    ) -> R
    where
        FIO: FnOnce(IO, A) -> R,
        FH: FnOnce(H, A) -> R,
        FS: FnOnce(S, A) -> R,
    {
        match self {
            MkStdio(io) => io_into(io, arg),
            MkHos(hos) => hos.project_into_with(arg, host_into, store_into),
        }
    }

    pub async fn await_futures(
        self,
    ) -> Endpoint<<IO as Future>::Output, <H as Future>::Output, <S as Future>::Output>
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

impl<T> Endpoint<T, T, T> {
    pub fn distill(self) -> T {
        match self {
            MkStdio(t) => t,
            MkHos(hos) => hos.distill(),
        }
    }
}

impl<IO, H, S, E> Endpoint<Result<IO, E>, Result<H, E>, Result<S, E>> {
    pub fn transpose(self) -> Result<Endpoint<IO, H, S>, E> {
        match self {
            MkStdio(io) => io.map(MkStdio),
            MkHos(hos) => hos.transpose().map(MkHos),
        }
    }
}

impl<IO, H, S> AsyncRead for Endpoint<IO, H, S>
where
    IO: AsyncRead,
    HostOrStore<H, S>: AsyncRead,
{
    fn poll_read(
        self: std::pin::Pin<&mut Self>,
        cx: &mut std::task::Context<'_>,
        buf: &mut tokio::io::ReadBuf<'_>,
    ) -> std::task::Poll<std::io::Result<()>> {
        use EndpointProjection::*;

        match self.project() {
            MkStdio(io) => io.poll_read(cx, buf),
            MkHos(hos) => hos.poll_read(cx, buf),
        }
    }
}

impl<IO, H, S> From<HostOrStore<H, S>> for Endpoint<IO, H, S> {
    fn from(hos: HostOrStore<H, S>) -> Self {
        MkHos(hos)
    }
}

impl<IO, H, S> fmt::Display for Endpoint<IO, H, S>
where
    IO: fmt::Display,
    H: fmt::Display,
    S: fmt::Display,
{
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        self.as_ref()
            .map_io(|io| io.fmt(f))
            .map_host(|h| h.fmt(f))
            .map_store(|t| t.fmt(f))
            .transpose()
            .map(Endpoint::distill)
    }
}

impl<IO, H, S> FromStr for Endpoint<IO, H, S>
where
    IO: FromStr<Err = anyhow::Error>,
    H: FromStr<Err = anyhow::Error>,
    S: FromStr<Err = anyhow::Error>,
{
    type Err = anyhow::Error;

    fn from_str(s: &str) -> Result<Self> {
        use Endpoint::*;

        if s == "-" {
            s.parse().map(MkStdio)
        } else {
            s.parse().map(MkHos)
        }
    }
}
