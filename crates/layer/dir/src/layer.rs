use anyhow::Result;
use pangalactic_iowrappers::Readable;
use pangalactic_link::Link;
use pangalactic_linkkind::LinkKind;
use pangalactic_store::{Commit, Load, Store};

use crate::Writer;

#[derive(Debug, Default, derive_more::From)]
pub struct LinkDirectoryLayer<S>(S)
where
    S: Store;

impl<S> Store for LinkDirectoryLayer<S>
where
    S: Store,
{
    type CID = Link<S::CID>;
    type Reader = Readable<S::Reader>;
    type Writer = Writer<S::Writer>;

    async fn open_writer(&self) -> Result<Self::Writer> {
        self.open_link_writer(LinkKind::File).await
    }
}

impl<S> LinkDirectoryLayer<S>
where
    S: Store,
{
    pub(crate) async fn commit_inner<T>(
        &mut self,
        kind: LinkKind,
        object: T,
    ) -> Result<Link<S::CID>>
    where
        T: Commit<S> + Send,
    {
        let cid = self.0.commit(object).await?;
        Ok(Link::new(kind, cid))
    }

    pub(crate) async fn open_link_writer(&self, kind: LinkKind) -> Result<Writer<S::Writer>> {
        let writer = self.0.open_writer().await?;
        Ok(Writer::new(kind, writer))
    }

    pub(crate) async fn open_kind_reader(
        &self,
        link: &Link<S::CID>,
        expected: LinkKind,
    ) -> Result<Readable<S::Reader>> {
        let (kind, reader) = self.open_any_reader(link).await?;
        kind.require_kind(expected)?;
        Ok(reader)
    }

    pub(crate) async fn open_any_reader(
        &self,
        link: &Link<S::CID>,
    ) -> Result<(LinkKind, Readable<S::Reader>)> {
        let kind = link.kind();
        let inner: S::Reader = self.0.load(link.peek_cid()).await?;
        Ok((kind, Readable(inner)))
    }
}

impl<S> Load<LinkDirectoryLayer<S>> for Readable<S::Reader>
where
    S: Store,
{
    async fn load_from_store(store: &LinkDirectoryLayer<S>, link: &Link<S::CID>) -> Result<Self> {
        store.open_kind_reader(link, LinkKind::File).await
    }
}
