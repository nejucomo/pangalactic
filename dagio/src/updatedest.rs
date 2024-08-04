use async_trait::async_trait;
use pangalactic_hostdir::HostDirectory;
use pangalactic_layer_cidmeta::CidMeta;
use pangalactic_link::Link;
use pangalactic_store::Store;
use pangalactic_path::StoreDestination;

use crate::Dagio;

#[cfg_attr(not(doc), async_trait)]
pub trait DagioUpdateDestination<S>
where
    S: Store,
{
    async fn update_link(
        self,
        dagio: &mut Dagio<S>,
        link: Link<CidMeta<S::CID>>,
    ) -> anyhow::Result<Link<CidMeta<S::CID>>>;
}

#[cfg_attr(not(doc), async_trait)]
impl<'a, S> DagioUpdateDestination<S> for Option<&'a StoreDestination<CidMeta<S::CID>>>
where
    S: Store,
{
    async fn update_link(
        self,
        dagio: &mut Dagio<S>,
        link: Link<CidMeta<S::CID>>,
    ) -> anyhow::Result<Link<CidMeta<S::CID>>> {
        if let Some(dest) = self {
            dest.update_link(dagio, link).await
        } else {
            Ok(link)
        }
    }
}

#[cfg_attr(not(doc), async_trait)]
impl<'a, S> DagioUpdateDestination<S> for &'a StoreDestination<CidMeta<S::CID>>
where
    S: Store,
{
    async fn update_link(
        self,
        dagio: &mut Dagio<S>,
        mut link: Link<CidMeta<S::CID>>,
    ) -> anyhow::Result<Link<CidMeta<S::CID>>> {
        let mut dirlink = self.link().clone();
        let mut stack = vec![];
        let (last, intermediate) = self.path().split_last();

        for name in intermediate {
            let d: HostDirectory<S::CID> = dagio.load(&dirlink).await?;
            dirlink = d.get_required(name)?.clone();
            stack.push((d, name));
        }

        let mut d: HostDirectory<S::CID> = dagio.load(&dirlink).await?;
        d.insert(last.clone(), link)?;

        for (mut prevd, name) in stack.into_iter().rev() {
            link = dagio.commit(d).await?;
            prevd.overwrite(name.clone(), link);
            d = prevd;
        }

        dagio.commit(d).await
    }
}
