use dagwasm_dagio::{Dagio, FromDag, LinkFor, ToDag};
use dagwasm_dir::Directory;
use dagwasm_store::Store;
use std::collections::BTreeMap;

#[derive(Clone, Debug, Eq, PartialEq)]
pub enum MemTree {
    File(Vec<u8>),
    Dir(BTreeMap<String, MemTree>),
}
use MemTree::*;

impl<'a, const K: usize> From<&'a [u8; K]> for MemTree {
    fn from(bytes: &'a [u8; K]) -> Self {
        File(Vec::from(bytes.as_slice()))
    }
}

impl<'a, const K: usize> From<[(&'a str, MemTree); K]> for MemTree {
    fn from(members: [(&'a str, MemTree); K]) -> Self {
        MemTree::Dir(
            members
                .into_iter()
                .map(|(s, m)| (s.to_string(), m))
                .collect(),
        )
    }
}

#[async_trait::async_trait]
impl<S> ToDag<S> for MemTree
where
    S: Store,
{
    async fn into_dag(self, dagio: &mut Dagio<S>) -> anyhow::Result<LinkFor<S>> {
        use MemTree::*;

        match self {
            File(bytes) => dagio.write_file(&bytes).await,
            Dir(entries) => {
                let mut d = Directory::default();
                for (n, child) in entries {
                    let link = child.clone().into_dag(dagio).await?;
                    d.insert(n.to_string(), link)?;
                }
                d.into_dag(dagio).await
            }
        }
    }
}

#[async_trait::async_trait]
impl<S> FromDag<S> for MemTree
where
    S: Store,
{
    async fn from_dag(dagio: &mut Dagio<S>, link: &LinkFor<S>) -> anyhow::Result<Self> {
        use dagwasm_linkkind::LinkKind as LK;

        match link.kind() {
            LK::File => dagio.read_file(link).await.map(File),
            LK::Dir => {
                let mut map = BTreeMap::default();
                let d: Directory<_> = dagio.read(link).await?;
                for (n, sublink) in d {
                    let mt: MemTree = dagio.read(&sublink).await?;
                    map.insert(n, mt);
                }
                Ok(Dir(map))
            }
        }
    }
}
