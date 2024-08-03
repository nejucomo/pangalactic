use async_trait::async_trait;
use pangalactic_hostdir::HostDirectoryLayer;
use pangalactic_store::Store;

use crate::StorePath;

#[derive(Debug, Default, derive_more::From)]
pub struct PathLayer<S>(HostDirectoryLayer<S>)
where
    S: Store;

#[async_trait]
impl<S> Store for PathLayer<S>
where
    S: Store,
{
    type CID = StorePath<S::CID>;
    type Reader = Reader<S::Reader>;
    type Writer = Writer<S::Writer>;

    async fn open_writer(&self) -> anyhow::Result<Self::Writer> {
        self.open_link_writer(LinkKind::File).await
    }
}
