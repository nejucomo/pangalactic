use dagwasm_dagio::{Dagio, LinkFor, ToDag};
use dagwasm_dir::Directory;
use dagwasm_store::Store;

#[derive(Clone, Debug, Eq, PartialEq)]
pub enum MemTree {
    File(Vec<u8>),
    Dir(Vec<(String, MemTree)>),
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
