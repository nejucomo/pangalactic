use pangalactic_layer_dir::Name;
use pangalactic_store::{Load, Store};

use crate::store::{HostDir, HostLayer, HostLink};

#[derive(Debug)]
pub(crate) struct DirectoryReader<S>
where
    S: Store,
{
    iter: <HostDir<S::CID> as IntoIterator>::IntoIter,
    name: Option<Name>,
    link: Option<HostLink<S::CID>>,
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

    pub(crate) fn take_link(&mut self) -> anyhow::Result<HostLink<S::CID>> {
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

impl<S> Load<HostLayer<S>> for DirectoryReader<S>
where
    S: Store,
{
    async fn load_from_store(
        store: &HostLayer<S>,
        link: &HostLink<S::CID>,
    ) -> anyhow::Result<Self> {
        let dir: HostDir<S::CID> = store.load(link).await?;
        let mut dr = DirectoryReader {
            iter: dir.into_iter(),
            name: None,
            link: None,
        };
        dr.next_entry();
        Ok(dr)
    }
}
