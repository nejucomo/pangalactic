use derive_more::{Deref, DerefMut, From, Into};
use pangalactic_dir::Directory;
pub use pangalactic_dir::DirectoryIntoIter;
use pangalactic_link::Link;
use pangalactic_linkkind::LinkKind;
use pangalactic_name::Name;
use pangalactic_serialization::{deserialize, serialize};
use pangalactic_store::{Commit, Load, Store};
use serde::{de::DeserializeOwned, Deserialize, Serialize};
use tokio::io::AsyncRead;

use crate::{LinkDirectoryLayer, LinkDirectorySerializationContainer};

#[derive(Clone, Deref, DerefMut, From, Into, Deserialize, Serialize, PartialEq)]
#[serde(
    bound = "C: Clone + DeserializeOwned + Serialize",
    try_from = "LinkDirectorySerializationContainer<C>",
    into = "LinkDirectorySerializationContainer<C>"
)]
pub struct LinkDirectory<C>(pub(crate) Inner<C>);

impl<C> LinkDirectory<C> {
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

impl<C> Default for LinkDirectory<C> {
    fn default() -> Self {
        LinkDirectory(Directory::default())
    }
}

impl<N, C> FromIterator<(N, Link<C>)> for LinkDirectory<C>
where
    Name: From<N>,
{
    fn from_iter<T>(iter: T) -> Self
    where
        T: IntoIterator<Item = (N, Link<C>)>,
    {
        LinkDirectory(Directory::from_iter(iter))
    }
}

impl<C> IntoIterator for LinkDirectory<C> {
    type Item = (Name, Link<C>);
    type IntoIter = DirectoryIntoIter<Link<C>>;

    fn into_iter(self) -> Self::IntoIter {
        self.0.into_iter()
    }
}

impl<S> Commit<LinkDirectoryLayer<S>> for LinkDirectory<S::CID>
where
    S: Store,
{
    async fn commit_into_store(
        self,
        store: &mut LinkDirectoryLayer<S>,
    ) -> anyhow::Result<Link<S::CID>> {
        use tokio::io::AsyncWriteExt;

        let buf = serialize(&self)?;

        let mut w = store.open_link_writer(LinkKind::Dir).await?;
        w.write_all(&buf).await?;
        store.commit(w).await
    }
}

impl<S> Load<LinkDirectoryLayer<S>> for LinkDirectory<S::CID>
where
    S: Store,
{
    async fn load_from_store(
        store: &LinkDirectoryLayer<S>,
        link: &Link<S::CID>,
    ) -> anyhow::Result<Self> {
        let reader = store.open_kind_reader(link, LinkKind::Dir).await?;
        LinkDirectory::deserialize_from(reader).await
    }
}

impl<C> std::fmt::Debug for LinkDirectory<C>
where
    C: Serialize,
{
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "{:?}", self.0)
    }
}
