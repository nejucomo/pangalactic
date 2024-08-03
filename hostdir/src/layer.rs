use async_trait::async_trait;
use pangalactic_link::Link;
use pangalactic_linkkind::LinkKind;
use pangalactic_store::{Commit, Store};

use crate::{Reader, Writer};

#[derive(Debug, Default, derive_more::From)]
pub struct HostDirectoryLayer<S>(S)
where
    S: Store;

#[async_trait]
impl<S> Store for HostDirectoryLayer<S>
where
    S: Store,
{
    type CID = Link<S::CID>;
    type Reader = Reader<S::Reader>;
    type Writer = Writer<S::Writer>;

    async fn open_writer(&self) -> anyhow::Result<Self::Writer> {
        self.open_link_writer(LinkKind::File).await
    }
}

impl<S> HostDirectoryLayer<S>
where
    S: Store,
{
    pub(crate) async fn commit_inner<T>(
        &mut self,
        kind: LinkKind,
        object: T,
    ) -> anyhow::Result<Link<S::CID>>
    where
        T: Commit<S> + Send,
    {
        let cid = self.0.commit(object).await?;
        Ok(Link::new(kind, cid))
    }

    pub(crate) async fn open_link_writer(
        &self,
        kind: LinkKind,
    ) -> anyhow::Result<Writer<S::Writer>> {
        let writer = self.0.open_writer().await?;
        Ok(Writer::new(kind, writer))
    }

    pub(crate) async fn open_kind_reader(
        &self,
        link: &Link<S::CID>,
        expected: LinkKind,
    ) -> anyhow::Result<Reader<S::Reader>> {
        let (kind, reader) = self.open_any_reader(link).await?;
        kind.require_kind(expected)?;
        Ok(reader)
    }

    pub(crate) async fn open_any_reader(
        &self,
        link: &Link<S::CID>,
    ) -> anyhow::Result<(LinkKind, Reader<S::Reader>)> {
        let kind = link.kind();
        let inner: S::Reader = self.0.load(link.peek_cid()).await?;
        Ok((kind, Reader::new(inner)))
    }
}
