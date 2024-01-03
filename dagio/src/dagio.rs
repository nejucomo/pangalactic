use crate::{
    DagioCommit, DagioHostDirectory, DagioLink, DagioLoad, DagioStoreDestination, DagioStorePath,
    DagioWriter,
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
    pub async fn load_from<T>(&mut self, source: &DagioStorePath<S>) -> anyhow::Result<T>
    where
        T: DagioLoad<S>,
    {
        let mut link = source.link().clone();
        for name in source.path() {
            let mut d: DagioHostDirectory<S> = self.load(&link).await?;
            link = d
                .remove(name)
                .ok_or_else(|| anyhow::anyhow!("missing {name:?} in {source}"))?;
        }
        self.load(&link).await
    }

    pub async fn commit_to<T>(
        &mut self,
        dest: &DagioStoreDestination<S>,
        value: T,
    ) -> anyhow::Result<DagioLink<S>>
    where
        T: DagioCommit<S>,
    {
        let (last, intermediates) = dest.path().split_last();
        let mut link = dest.link().clone();
        let mut d: DagioHostDirectory<S> = self.load(&link).await?;
        let mut stack = vec![];
        for name in intermediates {
            link = d
                .get(name)
                .cloned()
                .ok_or_else(|| anyhow::anyhow!("missing {name:?} in {dest}"))?;
            stack.push((d, name.clone()));
            d = self.load(&link).await?;
        }

        d = self.load(&link).await?;
        link = self.commit(value).await?;
        d.insert(last.to_string(), link)?;
        link = self.commit(d).await?;

        while let Some((mut d, name)) = stack.pop() {
            d.overwrite(name, link);
            link = self.commit(d).await?;
        }

        Ok(link)
    }

    pub async fn commit_to_opt<T>(
        &mut self,
        dest: Option<&DagioStoreDestination<S>>,
        value: T,
    ) -> anyhow::Result<DagioLink<S>>
    where
        T: DagioCommit<S>,
    {
        let link = self.commit(value).await?;
        if let Some(sd) = dest {
            self.commit_to(sd, link).await
        } else {
            Ok(link)
        }
    }

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
