use pangalactic_layer_dir::{StoreDirectory, StoreDirectoryLayer};
use pangalactic_link::Link;
use pangalactic_store::{Commit, Load, Store};
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

impl<S> Commit<StoreDirectoryLayer<S>> for MemTree
where
    S: Store,
{
    async fn commit_into_store(
        self,
        store: &mut StoreDirectoryLayer<S>,
    ) -> anyhow::Result<Link<S::CID>> {
        use MemTree::*;

        match self {
            File(bytes) => store.commit(bytes).await,
            Dir(entries) => {
                let mut d = StoreDirectory::default();
                for (n, child) in entries {
                    let link = Box::pin(store.commit(child.clone())).await?;
                    d.insert(n.to_string(), link)?;
                }
                store.commit(d).await
            }
        }
    }
}

impl<S> Load<StoreDirectoryLayer<S>> for MemTree
where
    S: Store,
{
    async fn load_from_store(
        store: &StoreDirectoryLayer<S>,
        link: &Link<S::CID>,
    ) -> anyhow::Result<Self> {
        use pangalactic_linkkind::LinkKind as LK;

        match link.kind() {
            LK::File => store.load(link).await.map(File),
            LK::Dir => {
                let mut map = BTreeMap::default();
                let d: StoreDirectory<_> = store.load(link).await?;
                for (n, sublink) in d {
                    let mt: MemTree = Box::pin(store.load(&sublink)).await?;
                    map.insert(n, mt);
                }
                Ok(Dir(map))
            }
        }
    }
}
