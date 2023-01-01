use async_trait::async_trait;
use dagwasm_blobstore::BlobStore;
use dagwasm_dagio::{Dagio, FromDag, LinkFor};
use dagwasm_dir::{Directory, Name};

#[derive(Debug)]
pub(crate) struct DirectoryReader<B>
where
    B: BlobStore,
{
    iter: <Directory<<B as BlobStore>::Key> as IntoIterator>::IntoIter,
    name: Option<Name>,
    link: Option<LinkFor<B>>,
}

impl<B> DirectoryReader<B>
where
    B: BlobStore,
{
    pub(crate) fn has_more_entries(&self) -> bool {
        self.name.is_some() || self.link.is_some()
    }

    pub(crate) fn take_name(&mut self) -> anyhow::Result<Name> {
        self.name
            .take()
            .ok_or_else(|| anyhow::Error::msg("name already taken in DirectoryReader"))
    }

    pub(crate) fn take_link(&mut self) -> anyhow::Result<LinkFor<B>> {
        self.link
            .take()
            .ok_or_else(|| anyhow::Error::msg("link already taken in DirectoryReader"))
    }

    pub(crate) fn next_entry(&mut self) {
        if let Some((name, link)) = self.iter.next() {
            self.name = Some(name);
            self.link = Some(link);
        } else {
            self.name = None;
            self.link = None;
        }
    }
}

#[async_trait]
impl<B> FromDag<B> for DirectoryReader<B>
where
    B: BlobStore,
{
    async fn from_dag(dagio: &mut Dagio<B>, link: &LinkFor<B>) -> anyhow::Result<Self> {
        let dir: Directory<<B as BlobStore>::Key> = dagio.read(link).await?;
        let mut dr = DirectoryReader {
            iter: dir.into_iter(),
            name: None,
            link: None,
        };
        dr.next_entry();
        Ok(dr)
    }
}
