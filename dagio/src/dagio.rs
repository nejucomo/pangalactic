use std::borrow::Borrow;

use crate::{
    aliases::DagioStoreDestination, DagioCommit, DagioHostDirectory, DagioLink, DagioLoad,
    DagioResolveLink, DagioWriter,
};
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
    pub async fn load<L, T>(&self, link: L) -> anyhow::Result<T>
    where
        L: DagioResolveLink<S>,
        T: DagioLoad<S>,
    {
        let linkproxy = link.resolve_link(self).await?;
        T::load_from_dagio(self, linkproxy.borrow()).await
    }

    pub async fn commit<T>(&mut self, object: T) -> anyhow::Result<DagioLink<S>>
    where
        T: DagioCommit<S>,
    {
        object.commit_into_dagio(self).await
    }

    pub async fn commit_into<T>(
        &mut self,
        object: T,
        dest: &DagioStoreDestination<S>,
    ) -> anyhow::Result<DagioLink<S>>
    where
        T: DagioCommit<S>,
    {
        let mut dirlink = dest.link().clone();
        let mut stack = vec![];

        for name in dest.path() {
            let d: DagioHostDirectory<S> = self.load(&dirlink).await?;
            dirlink = d.get_required(name)?.clone();
            stack.push((d, name));
        }

        let mut newlink = self.commit(object).await?;

        for (mut d, name) in stack.into_iter().rev() {
            d.overwrite(name.clone(), newlink);
            newlink = self.commit(d).await?;
        }

        Ok(newlink)
    }

    pub async fn open_file_writer(&mut self) -> anyhow::Result<DagioWriter<S>> {
        self.0.open_writer().await.map(DagioWriter::new)
    }
}
