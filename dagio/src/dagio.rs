use crate::{FromDag, LinkFor, ToDag};
use dagwasm_link::Link;
use dagwasm_linkkind::LinkKind::File;
use dagwasm_store::Store;

#[derive(Debug, derive_more::From)]
pub struct Dagio<S>(S);

impl<S> Dagio<S>
where
    S: Store,
{
    pub async fn read<T>(&mut self, link: &LinkFor<S>) -> anyhow::Result<T>
    where
        T: FromDag<S>,
    {
        T::from_dag(self, link).await
    }

    pub async fn commit<T>(&mut self, object: T) -> anyhow::Result<LinkFor<S>>
    where
        T: ToDag<S>,
    {
        object.into_dag(self).await
    }

    pub async fn open_file_reader(
        &mut self,
        link: &LinkFor<S>,
    ) -> anyhow::Result<<S as Store>::Reader> {
        let key = link.peek_key(File)?;
        self.0.open_reader(key).await
    }

    pub async fn open_file_writer(&mut self) -> anyhow::Result<<S as Store>::Writer> {
        self.0.open_writer().await
    }

    pub async fn commit_file_writer(
        &mut self,
        w: <S as Store>::Writer,
    ) -> anyhow::Result<LinkFor<S>> {
        self.0.commit_writer(w).await.map(|k| Link::new(File, k))
    }

    pub async fn read_file(&mut self, link: &LinkFor<S>) -> anyhow::Result<Vec<u8>> {
        let key = link.peek_key(File)?;
        self.0.read(key).await
    }

    pub async fn write_file(&mut self, contents: &[u8]) -> anyhow::Result<LinkFor<S>> {
        self.0.write(contents).await.map(|k| Link::new(File, k))
    }
}
