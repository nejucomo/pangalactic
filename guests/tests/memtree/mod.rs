use pangalactic_layer_storedir::StoreDirectory;
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

impl<S> Commit<S> for MemTree
where
    S: Store,
{
    async fn commit_into_store(self, store: &mut S) -> anyhow::Result<<S as Store>::CID> {
        use MemTree::*;

        match self {
            File(bytes) => dagio.commit(bytes).await,
            Dir(entries) => {
                let mut d = StoreDirectory::default();
                for (n, child) in entries {
                    let link = dagio.commit(child.clone()).await?;
                    d.insert(n.to_string(), link)?;
                }
                dagio.commit(d).await
            }
        }
    }
}

impl<S> Load<S> for MemTree
where
    S: Store,
{
    async fn load_from_store(store: &S, cid: &<S as Store>::CID) -> anyhow::Result<Self> {
        use pangalactic_linkkind::LinkKind as LK;

        match link.kind() {
            LK::File => dagio.load(link).await.map(File),
            LK::Dir => {
                let mut map = BTreeMap::default();
                let d: StoreDirectory<_> = dagio.load(link).await?;
                for (n, sublink) in d {
                    let mt: MemTree = dagio.load(&sublink).await?;
                    map.insert(n, mt);
                }
                Ok(Dir(map))
            }
        }
    }
}
