use anyhow::Result;
use include_dir::{include_dir, Dir};
use pangalactic_config::datapath;
use pangalactic_layer_dir::{LinkDirectory, LinkDirectoryLayer};
use pangalactic_link::Link;
use pangalactic_store::{Commit, Store};

use crate::{libderive::LibDerive, log};

#[derive(Debug, Copy, Clone)]
pub struct Seed;

impl Seed {
    pub async fn install<S>(self, store: &mut LinkDirectoryLayer<S>) -> Result<Link<S::CID>>
    where
        S: Store,
    {
        let link = store.commit(self).await?;
        let linkpath = datapath::get("seed.link");
        tracing::info!(
            "Remembering seed link {} in {:?}",
            &link,
            linkpath.display()
        );
        tokio::fs::write(&linkpath, link.to_string()).await?;
        Ok(link)
    }
}

static STATIC_DIR: Dir<'_> = include_dir!("$CARGO_MANIFEST_DIR/staticdir/");

impl<S> Commit<LinkDirectoryLayer<S>> for Seed
where
    S: Store,
{
    async fn commit_into_store(self, store: &mut LinkDirectoryLayer<S>) -> Result<Link<S::CID>> {
        let link = store.commit(&STATIC_DIR).await?;
        let mut toplevel: LinkDirectory<_> = store.load(&link).await?;
        let link = store.commit(LibDerive).await?;
        log::trace_insert("", &mut toplevel, "libderive", link)?;
        let link = store.commit(toplevel).await?;
        tracing::debug!("committed to {} seed dir", &link);
        Ok(link)
    }
}
