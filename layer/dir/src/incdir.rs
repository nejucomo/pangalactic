use crate::{LinkDirectory, LinkDirectoryLayer};
use anyhow::Result;
use anyhow_std::{OsStrAnyhow, PathAnyhow};
use include_dir::{Dir, DirEntry};
use pangalactic_link::Link;
use pangalactic_store::{Commit, Store};

impl<'a, 'b, S> Commit<LinkDirectoryLayer<S>> for &'a Dir<'b>
where
    S: Store,
{
    async fn commit_into_store(self, store: &mut LinkDirectoryLayer<S>) -> Result<Link<S::CID>> {
        let mut ld = LinkDirectory::default();
        for entry in self.entries() {
            let path = entry.path();
            let name = path.file_name_anyhow()?.to_str_anyhow()?;
            let link = store.commit(entry).await?;
            tracing::debug!("committed to {} static entry {:?}", &link, path.display());
            ld.insert(name.to_string(), link)?;
        }
        store.commit(ld).await
    }
}

impl<'a, 'b, S> Commit<LinkDirectoryLayer<S>> for &'a DirEntry<'b>
where
    S: Store,
{
    async fn commit_into_store(self, store: &mut LinkDirectoryLayer<S>) -> Result<Link<S::CID>> {
        use include_dir::DirEntry::*;

        match self {
            Dir(d) => Box::pin(store.commit(d)).await,
            File(f) => store.commit(f.contents()).await,
        }
    }
}
