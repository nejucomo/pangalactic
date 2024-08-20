use anyhow::Result;
use pangalactic_layer_dir::{LinkDirectory, LinkDirectoryLayer};
use pangalactic_store::{Commit, Store};

use crate::{libderive::LibDerive, log, staticdir::StaticDir};

pub struct Seed;

impl<S> Commit<LinkDirectoryLayer<S>> for Seed
where
    S: Store,
{
    async fn commit_into_store(
        self,
        store: &mut LinkDirectoryLayer<S>,
    ) -> Result<<LinkDirectoryLayer<S> as Store>::CID> {
        let mut toplevel = LinkDirectory::default();
        let link = store.commit(LibDerive).await?;
        log::trace_insert("", &mut toplevel, "libderive", link)?;
        let link = store.commit(StaticDir).await?;
        log::trace_insert("", &mut toplevel, "static", link)?;
        let link = store.commit(toplevel).await?;
        tracing::debug!("committed seed dir -> {}", &link);
        Ok(link)
    }
}
