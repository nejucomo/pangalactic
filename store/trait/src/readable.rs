use async_trait::async_trait;
use pin_project::pin_project;
use tokio::io::AsyncRead;

use crate::{Commit, Load, Store};

#[derive(Debug)]
#[pin_project]
pub struct Readable<R>(#[pin] pub R);

// #[cfg_attr(not(doc), async_trait)]
// impl<S> DagioLoad<S> for Readable<S>
// where
//     S: Store,
// {
//     async fn load_from_dagio(
//         dagio: &Dagio<S>,
//         link: &Link<CidMeta<S::CID>>,
//     ) -> anyhow::Result<Self> {
//         use pangalactic_linkkind::LinkKind::File;

//         let cid = link.peek_cid_kind(File)?;
//         dagio.0.open_reader(cid).await.map(Readable)
//     }
// }

impl<R> AsyncRead for Readable<R>
where
    R: AsyncRead,
{
    fn poll_read(
        self: std::pin::Pin<&mut Self>,
        cx: &mut std::task::Context<'_>,
        buf: &mut tokio::io::ReadBuf<'_>,
    ) -> std::task::Poll<std::io::Result<()>> {
        self.project().0.poll_read(cx, buf)
    }
}

#[cfg_attr(not(doc), async_trait)]
impl<S, R> Commit<S> for Readable<R>
where
    S: Store,
    R: AsyncRead + Send,
{
    async fn commit_into_store(mut self, store: &mut S) -> anyhow::Result<S::CID> {
        use std::pin::pin;
        use tokio::io;

        let mut w = store.open_writer().await?;
        io::copy(&mut pin!(self.0), &mut w).await?;
        store.commit(w).await
    }
}

#[cfg_attr(not(doc), async_trait)]
impl<S> Load<S> for Readable<S::Reader>
where
    S: Store,
{
    async fn load_from_store(store: &S, cid: &S::CID) -> anyhow::Result<Self> {
        let inner: S::Reader = store.load(cid).await?;
        Ok(Readable(inner))
    }
}
