use derive_more::{Deref, DerefMut, From, Into};
use pangalactic_dir::Directory;
use pangalactic_link::Link;
use pangalactic_linkkind::LinkKind;
use pangalactic_serialization::{deserialize, serialize};
use pangalactic_store::{Commit, Load, Store};
use serde::{de::DeserializeOwned, Deserialize, Serialize};
use tokio::io::AsyncRead;

use crate::{HostDirectoryLayer, HostDirectorySerializationContainer, Name};

#[derive(Clone, Debug, Deref, DerefMut, From, Into, Deserialize, Serialize, PartialEq)]
#[serde(
    bound = "C: Clone + DeserializeOwned + Serialize",
    try_from = "HostDirectorySerializationContainer<C>",
    into = "HostDirectorySerializationContainer<C>"
)]
pub struct HostDirectory<C>(pub(crate) Inner<C>);

impl<C> HostDirectory<C> {
    pub(crate) async fn deserialize_from<R>(reader: R) -> anyhow::Result<Self>
    where
        C: Clone + Serialize + DeserializeOwned,
        R: AsyncRead,
    {
        use tokio::io::AsyncReadExt;

        let mut bytes = vec![];
        std::pin::pin!(reader).read_to_end(&mut bytes).await?;
        let dir = deserialize(bytes)?;
        Ok(dir)
    }
}

pub(crate) type Inner<C> = Directory<Link<C>>;

impl<C> Default for HostDirectory<C> {
    fn default() -> Self {
        HostDirectory(Directory::default())
    }
}

impl<N, C> FromIterator<(N, Link<C>)> for HostDirectory<C>
where
    Name: From<N>,
{
    fn from_iter<T>(iter: T) -> Self
    where
        T: IntoIterator<Item = (N, Link<C>)>,
    {
        HostDirectory(Directory::from_iter(iter))
    }
}

impl<C> IntoIterator for HostDirectory<C> {
    type Item = (Name, Link<C>);
    type IntoIter = <Directory<Link<C>> as IntoIterator>::IntoIter;

    fn into_iter(self) -> Self::IntoIter {
        self.0.into_iter()
    }
}

impl<S> Commit<HostDirectoryLayer<S>> for HostDirectory<S::CID>
where
    S: Store,
{
    async fn commit_into_store(
        self,
        store: &mut HostDirectoryLayer<S>,
    ) -> anyhow::Result<Link<S::CID>> {
        use tokio::io::AsyncWriteExt;

        let buf = serialize(&self)?;

        let mut w = store.open_link_writer(LinkKind::Dir).await?;
        w.write_all(&buf).await?;
        store.commit(w).await
    }
}

impl<S> Load<HostDirectoryLayer<S>> for HostDirectory<S::CID>
where
    S: Store,
{
    async fn load_from_store(
        store: &HostDirectoryLayer<S>,
        link: &Link<S::CID>,
    ) -> anyhow::Result<Self> {
        let reader = store.open_kind_reader(link, LinkKind::Dir).await?;
        HostDirectory::deserialize_from(reader).await
    }
}
