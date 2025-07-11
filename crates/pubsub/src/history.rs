use pangalactic_layer_dir::{LinkDirectory, LinkDirectoryLayer};
use pangalactic_link::Link;
use pangalactic_store::{Commit, Store};
use serde::{Deserialize, Serialize};

#[derive(Debug, Serialize, Deserialize)]
pub struct History<C>
where
    C: Serialize,
{
    content: Link<C>,
    prev: Option<Link<C>>,
}

impl<S> Commit<LinkDirectoryLayer<S>> for History<S::CID>
where
    S: Store,
{
    async fn commit_into_store(
        self,
        store: &mut LinkDirectoryLayer<S>,
    ) -> anyhow::Result<Link<S::CID>> {
        let mut ld = LinkDirectory::default();
        ld.insert("content", self.content)?;

        if let Some(link) = self.prev {
            ld.insert("history", link)?;
        }

        let link = store.commit(ld).await?;
        Ok(link)
    }
}
