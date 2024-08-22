use anyhow::Result;
use include_dir::{include_dir, Dir};
use pangalactic_layer_dir::{LinkDirectory, LinkDirectoryLayer};
use pangalactic_store::{Commit, Store};

use crate::{libderive::LibDerive, log};

pub struct Seed;

static STATIC_DIR: Dir<'_> = include_dir!("$CARGO_MANIFEST_DIR/staticdir/");

impl<S> Commit<LinkDirectoryLayer<S>> for Seed
where
    S: Store,
{
    async fn commit_into_store(
        self,
        store: &mut LinkDirectoryLayer<S>,
    ) -> Result<<LinkDirectoryLayer<S> as Store>::CID> {
        let link = store.commit(&STATIC_DIR).await?;
        let mut toplevel: LinkDirectory<_> = store.load(&link).await?;
        let link = store.commit(LibDerive).await?;
        log::trace_insert("", &mut toplevel, "libderive", link)?;
        let link = store.commit(toplevel).await?;
        tracing::debug!("committed to {} seed dir", &link);
        Ok(link)
    }
}
