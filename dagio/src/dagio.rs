use crate::{FileWriter, FromDag, LinkFor, ToDag};
use dagwasm_dir::{Link, LinkKind::File};
use dagwasm_store::Store;

#[derive(Debug, derive_more::From)]
pub struct Dagio<S>(S);

impl<S> Dagio<S>
where
    S: Store,
    <S as Store>::Writer: Send + std::marker::Unpin,
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

    pub async fn open_file_writer(&mut self) -> anyhow::Result<FileWriter<<S as Store>::Writer>> {
        let inner = self.0.open_writer().await?;
        Ok(FileWriter::from(inner))
    }

    pub async fn commit_file_writer(
        &mut self,
        w: FileWriter<<S as Store>::Writer>,
    ) -> anyhow::Result<LinkFor<S>> {
        self.0
            .commit_writer(w.unwrap())
            .await
            .map(|k| Link::new(File, k))
    }

    pub async fn read_file(&mut self, link: &LinkFor<S>) -> anyhow::Result<Vec<u8>> {
        let key = link.peek_key(File)?;
        self.0.read(key).await
    }

    pub async fn write_file(&mut self, contents: &[u8]) -> anyhow::Result<LinkFor<S>> {
        self.0.write(contents).await.map(|k| Link::new(File, k))
    }
}
