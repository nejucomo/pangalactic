use pangalactic_dagio::{Dagio, DagioCommit, DagioLoad};
use pangalactic_hostdir::HostDirectory;
use pangalactic_layer_cidmeta::CidMeta;
use pangalactic_link::Link;
use pangalactic_store::Store;
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
impl<S> DagioCommit<S> for MemTree
where
    S: Store,
{
    async fn commit_into_dagio(
        self,
        dagio: &mut Dagio<S>,
    ) -> anyhow::Result<Link<CidMeta<S::CID>>> {
        use MemTree::*;

        match self {
            File(bytes) => dagio.commit(bytes).await,
            Dir(entries) => {
                let mut d = HostDirectory::default();
                for (n, child) in entries {
                    let link = dagio.commit(child.clone()).await?;
                    d.insert(n.to_string(), link)?;
                }
                dagio.commit(d).await
            }
        }
    }
}

#[async_trait::async_trait]
impl<S> DagioLoad<S> for MemTree
where
    S: Store,
{
    async fn load_from_dagio(
        dagio: &Dagio<S>,
        link: &Link<CidMeta<S::CID>>,
    ) -> anyhow::Result<Self> {
        use pangalactic_linkkind::LinkKind as LK;

        match link.kind() {
            LK::File => dagio.load(link).await.map(File),
            LK::Dir => {
                let mut map = BTreeMap::default();
                let d: HostDirectory<_> = dagio.load(link).await?;
                for (n, sublink) in d {
                    let mt: MemTree = dagio.load(&sublink).await?;
                    map.insert(n, mt);
                }
                Ok(Dir(map))
            }
        }
    }
}
