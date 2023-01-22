use crate::{CidMeta, Writer};
use async_trait::async_trait;
use dagwasm_store::Store;

#[derive(Debug, Default, derive_more::From)]
pub struct CidMetaLayer<S>(S)
where
    S: Store;

#[async_trait]
impl<S> Store for CidMetaLayer<S>
where
    S: Store,
{
    type CID = CidMeta<S>;
    type Reader = <S as Store>::Reader;
    type Writer = Writer<<S as Store>::Writer>;

    async fn open_reader(&mut self, key: &Self::CID) -> anyhow::Result<Self::Reader> {
        self.0.open_reader(&key.cid).await
    }

    async fn open_writer(&mut self) -> anyhow::Result<Self::Writer> {
        let writer = self.0.open_writer().await?;
        Ok(Writer { writer, written: 0 })
    }

    async fn commit_writer(
        &mut self,
        Writer { writer, written }: Self::Writer,
    ) -> anyhow::Result<Self::CID> {
        let cid = self.0.commit_writer(writer).await?;
        let size = u64::try_from(written).expect("usize->u64 conversion failure");
        Ok(CidMeta { cid, size })
    }
}
