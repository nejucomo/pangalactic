use anyhow::Result;
use extend::ext;
use pangalactic_layer_dir::{LinkDirectory, LinkDirectoryLayer};
use pangalactic_link::Link;
use pangalactic_store::{Commit, Load, Store};

use crate::{AnyDestination, AnySource, StoreDestination, StorePath};

#[ext(name = PathLayerExt)]
pub impl<S> LinkDirectoryLayer<S>
where
    S: Store,
{
    async fn transfer(
        &mut self,
        source: AnySource<S::CID>,
        destination: AnyDestination<S::CID>,
    ) -> Result<Option<StorePath<S::CID>>> {
        use crate::transfer::TransferInto;

        source.transfer_into(self, destination).await
    }

    async fn commit_into_optdest<T>(
        &mut self,
        value: T,
        optdest: Option<StoreDestination<<S as Store>::CID>>,
    ) -> Result<StorePath<<S as Store>::CID>>
    where
        T: Commit<LinkDirectoryLayer<S>> + Send,
    {
        if let Some(dest) = optdest {
            self.commit_into_dest(value, dest).await
        } else {
            self.commit(value).await.map(StorePath::from)
        }
    }

    async fn commit_into_dest<T>(
        &mut self,
        value: T,
        destination: StoreDestination<S::CID>,
    ) -> Result<StorePath<S::CID>>
    where
        T: Commit<LinkDirectoryLayer<S>> + Send,
    {
        let mut link = self.commit(value).await?;

        let mut dirlink = destination.link().clone();
        let mut stack = vec![];
        let (last, intermediate) = destination.path().split_last();

        for name in intermediate {
            let d: LinkDirectory<S::CID> = self.load(&dirlink).await?;
            dirlink = d.get_required(name)?.clone();
            stack.push((d, name));
        }

        let mut d: LinkDirectory<S::CID> = self.load(&dirlink).await?;
        d.insert(last.clone(), link)?;

        for (mut prevd, name) in stack.into_iter().rev() {
            link = self.commit(d).await?;
            prevd.overwrite(name.to_owned(), link);
            d = prevd;
        }

        let newroot = self.commit(d).await?;
        destination.replace_link_into_path(newroot)
    }

    async fn load_path<T>(&self, p: &StorePath<S::CID>) -> Result<T>
    where
        T: Load<LinkDirectoryLayer<S>>,
    {
        let link = self.resolve_path(p).await?;
        self.load(&link).await
    }

    async fn resolve_path(&self, p: &StorePath<S::CID>) -> Result<Link<S::CID>> {
        let mut link = p.link().clone();
        for name in p.path() {
            let mut d: LinkDirectory<S::CID> = self.load(&link).await?;
            link = d.remove_required(name)?;
        }
        Ok(link)
    }
}
