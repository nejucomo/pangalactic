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
        let (last, intermediate) = self.path().split_last();

        for name in intermediate {
            let d: DagioHostDirectory<S> = dagio.load(&dirlink).await?;
            dirlink = d.get_required(name)?.clone();
            stack.push((d, name));
        }

        let mut d: DagioHostDirectory<S> = dagio.load(&dirlink).await?;
        d.insert(last.clone(), link)?;

        for (mut prevd, name) in stack.into_iter().rev() {
            link = dagio.commit(d).await?;
            prevd.overwrite(name.clone(), link);
            d = prevd;
        }

        dagio.commit(d).await
    }
}
