use async_trait::async_trait;
use pangalactic_dagio::{Dagio, FromDag, HostDirectoryFor, LinkFor};
use pangalactic_hostdir::Name;
use pangalactic_store::Store;

#[derive(Debug)]
pub(crate) struct DirectoryReader<S>
where
    S: Store,
{
    iter: <HostDirectoryFor<S> as IntoIterator>::IntoIter,
    name: Option<Name>,
    link: Option<LinkFor<S>>,
}

impl<S> DirectoryReader<S>
where
    S: Store,
{
    pub(crate) fn has_more_entries(&self) -> bool {
        self.name.is_some() || self.link.is_some()
    }

    pub(crate) fn take_name(&mut self) -> anyhow::Result<Name> {
        self.name
            .take()
            .ok_or_else(|| anyhow::Error::msg("name already taken in DirectoryReader"))
    }

    pub(crate) fn take_link(&mut self) -> anyhow::Result<LinkFor<S>> {
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
impl<S> FromDag<S> for DirectoryReader<S>
where
    S: Store,
{
    async fn from_dag(dagio: &mut Dagio<S>, link: &LinkFor<S>) -> anyhow::Result<Self> {
        let dir: HostDirectoryFor<S> = dagio.read(link).await?;
        let mut dr = DirectoryReader {
            iter: dir.into_iter(),
            name: None,
            link: None,
        };
        dr.next_entry();
        Ok(dr)
    }
}
