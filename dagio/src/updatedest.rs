use async_trait::async_trait;
use pangalactic_store::Store;

use crate::{Dagio, DagioHostDirectory, DagioLink, DagioStoreDestination};

#[cfg_attr(not(doc), async_trait)]
pub trait DagioUpdateDestination<S>
where
    S: Store,
{
    async fn update_link(
        self,
        dagio: &mut Dagio<S>,
        link: DagioLink<S>,
    ) -> anyhow::Result<DagioLink<S>>;
}

#[cfg_attr(not(doc), async_trait)]
impl<'a, S> DagioUpdateDestination<S> for Option<&'a DagioStoreDestination<S>>
where
    S: Store,
{
    async fn update_link(
        self,
        dagio: &mut Dagio<S>,
        link: DagioLink<S>,
    ) -> anyhow::Result<DagioLink<S>> {
        if let Some(dest) = self {
            dest.update_link(dagio, link).await
        } else {
            Ok(link)
        }
    }
}

#[cfg_attr(not(doc), async_trait)]
impl<'a, S> DagioUpdateDestination<S> for &'a DagioStoreDestination<S>
where
    S: Store,
{
    async fn update_link(
        self,
        dagio: &mut Dagio<S>,
        mut link: DagioLink<S>,
    ) -> anyhow::Result<DagioLink<S>> {
        let mut dirlink = self.link().clone();
        let mut stack = vec![];

        for name in self.path() {
            let d: DagioHostDirectory<S> = dagio.load(&dirlink).await?;
            dirlink = d.get_required(name)?.clone();
            stack.push((d, name));
        }

        for (mut d, name) in stack.into_iter().rev() {
            d.overwrite(name.clone(), link);
            link = dagio.commit(d).await?;
        }

        Ok(link)
    }
}
