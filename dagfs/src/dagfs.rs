use pangalactic_dagio::{Dagio, FromDag, LinkFor, ToDag, WriterFor};
use pangalactic_hosttree::{HostTree, HostTreeDestination, HostTreePath};
use pangalactic_store::Store;
use tokio::io::AsyncRead;

#[derive(Debug, Default, derive_more::Deref, derive_more::DerefMut)]
pub struct Dagfs<S>(Dagio<S>)
where
    S: Store;

pub type DagfsPath<S> = HostTreePath<S>;
pub type DagfsDestination<S> = HostTreeDestination<S>;

impl<S, D> From<D> for Dagfs<S>
where
    S: Store,
    Dagio<S>: From<D>,
{
    fn from(d: D) -> Self {
        Dagfs(Dagio::from(d))
    }
}

impl<S> Dagfs<S>
where
    S: Store,
{
    /// Read the object from `source`
    pub async fn read_path<T>(&mut self, source: &DagfsPath<S>) -> anyhow::Result<T>
    where
        T: FromDag<S>,
    {
        HostTree::read_path(&mut self.0, source).await
    }

    /// Open a file reader from `source`
    pub async fn open_path_file_reader(
        &mut self,
        source: &DagfsPath<S>,
    ) -> anyhow::Result<S::Reader> {
        // TODO: Can we make `Store::Reader` impl `FromDag` to remove the need for this category of methods throughout the stack?
        let link = HostTree::read_path(&mut self.0, source).await?;
        self.open_file_reader(&link).await
    }

    /// Set `dest` to point to `object`
    ///
    /// The final component of `dest` is the name to associate to `object`. It is an error for `dest` to not have any name components (i.e. be a link without any path suffix.
    ///
    /// The returned link points to a new root with equivalent tree structure as `dest`, with the new `object` linked at the new path.
    pub async fn commit_path<T>(
        &mut self,
        dest: &DagfsDestination<S>,
        object: T,
    ) -> anyhow::Result<LinkFor<S>>
    where
        T: ToDag<S>,
    {
        HostTree::set_path(&mut self.0, dest, object).await
    }

    pub async fn commit_file_from_reader_to_path<R>(
        &mut self,
        dest: &DagfsDestination<S>,
        r: R,
    ) -> anyhow::Result<LinkFor<S>>
    where
        R: AsyncRead,
    {
        let link = self.commit_file_from_reader(r).await?;
        HostTree::set_path(&mut self.0, dest, link).await
    }

    pub async fn commit_file_writer_to_path(
        &mut self,
        dest: &DagfsDestination<S>,
        w: WriterFor<S>,
    ) -> anyhow::Result<LinkFor<S>> {
        let link = self.commit_file_writer(w).await?;
        HostTree::set_path(&mut self.0, dest, link).await
    }
}
