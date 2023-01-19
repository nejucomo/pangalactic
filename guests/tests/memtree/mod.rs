use dagwasm_dagio::{Dagio, LinkFor, ToDag};
use dagwasm_dir::Directory;
use dagwasm_store::Store;

#[derive(Clone, Debug)]
pub enum MemTree {
    File(&'static [u8]),
    Dir(&'static [(&'static str, MemTree)]),
}

#[async_trait::async_trait]
impl<S> ToDag<S> for MemTree
where
    S: Store,
{
    async fn into_dag(self, dagio: &mut Dagio<S>) -> anyhow::Result<LinkFor<S>> {
        use MemTree::*;

        match self {
            File(bytes) => dagio.write_file(bytes).await,
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
