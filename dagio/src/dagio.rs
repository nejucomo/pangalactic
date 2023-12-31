use crate::{DagioCommit, DagioLink, DagioLoad, DagioWriter};
use pangalactic_layer_cidmeta::CidMetaLayer;
use pangalactic_store::Store;

#[derive(Debug, Default)]
pub struct Dagio<S>(pub(crate) CidMetaLayer<S>)
where
    S: Store;

impl<S> From<S> for Dagio<S>
where
    S: Store,
{
    fn from(store: S) -> Self {
        Dagio(CidMetaLayer::from(store))
    }
}

impl<S> Dagio<S>
where
    S: Store,
{
    pub async fn load<T>(&mut self, link: &DagioLink<S>) -> anyhow::Result<T>
    where
        T: DagioLoad<S>,
    {
        T::load_from_dagio(self, link).await
    }

    pub async fn commit<T>(&mut self, object: T) -> anyhow::Result<DagioLink<S>>
    where
        T: DagioCommit<S>,
    {
        object.commit_into_dagio(self).await
    }

    pub async fn open_file_writer(&mut self) -> anyhow::Result<DagioWriter<S>> {
        self.0.open_writer().await.map(DagioWriter::new)
    }
}
