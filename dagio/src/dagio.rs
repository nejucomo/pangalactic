use crate::{Link, LinkKind};
use dagwasm_blobstore::BlobStore;

#[derive(Debug, derive_more::From)]
pub struct Dagio<B>(B);

impl<B> Dagio<B>
where
    B: BlobStore,
{
    pub async fn open_file_reader(
        &mut self,
        link: Link<<B as BlobStore>::Key>,
    ) -> anyhow::Result<<B as BlobStore>::Reader> {
        let key = link.unwrap_key(LinkKind::File)?;
        self.0.open_reader(key).await
    }
}
