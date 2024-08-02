use async_trait::async_trait;
use pangalactic_store::Store;

use crate::{CidMeta, Reader, Writer};

#[derive(Debug, Default, derive_more::From)]
pub struct CidMetaLayer<S>(pub(crate) S)
where
    S: Store;

#[async_trait]
impl<S> Store for CidMetaLayer<S>
where
    S: Store,
{
    type CID = CidMeta<S::CID>;
    type Reader = Reader<S::Reader>;
    type Writer = Writer<<S as Store>::Writer>;

    async fn open_writer(&self) -> anyhow::Result<Self::Writer> {
        let writer = self.0.open_writer().await?;
        Ok(Writer { writer, written: 0 })
    }
}
