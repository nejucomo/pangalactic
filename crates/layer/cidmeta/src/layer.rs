use pangalactic_iowrappers::Readable;
use pangalactic_store::{Load, Store};
use serde::{de::DeserializeOwned, Serialize};

use crate::{CidMeta, Writer};

#[derive(Debug, Default, derive_more::From)]
pub struct CidMetaLayer<S>(pub(crate) S)
where
    S: Store;

impl<S> Store for CidMetaLayer<S>
where
    S: Store,
    S::CID: Serialize + DeserializeOwned,
{
    type CID = CidMeta<S::CID>;
    type Reader = Readable<S::Reader>;
    type Writer = Writer<S::Writer>;

    async fn open_writer(&self) -> anyhow::Result<Self::Writer> {
        self.0.open_writer().await.map(Writer::from)
    }
}

impl<S> Load<CidMetaLayer<S>> for Readable<S::Reader>
where
    S: Store,
    S::CID: Serialize + DeserializeOwned,
{
    async fn load_from_store(
        store: &CidMetaLayer<S>,
        cid: &CidMeta<S::CID>,
    ) -> anyhow::Result<Self> {
        store.0.load(&cid.cid).await.map(Readable)
    }
}
