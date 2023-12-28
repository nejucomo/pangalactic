use pangalactic_dagio::{Dagio, FromDag, HostDirectoryFor, LinkFor, ToDag, WriterFor};
use pangalactic_dir::Name;
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
        let link = self.commit(object).await?;
        self.set_path(dest, link).await
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
        self.set_path(dest, link).await
    }

    pub async fn commit_file_writer_to_path(
        &mut self,
        dest: &DagfsDestination<S>,
        w: WriterFor<S>,
    ) -> anyhow::Result<LinkFor<S>> {
        let link = self.commit_file_writer(w).await?;
        self.set_path(dest, link).await
    }

    async fn set_path(
        &mut self,
        dest: &DagfsDestination<S>,
        link: LinkFor<S>,
    ) -> anyhow::Result<LinkFor<S>> {
        // Scan down to the penultimate path component, saving tree structure and name path
        let (components, parentlink, lastname) = self.dest_components(dest).await?;

        // Insert `object`:
        let mut link = self.update_dir_link(&parentlink, lastname, link).await?;

        // Rebuild the structure with the new object:
        for (d, parentname) in components.into_iter().rev() {
            link = self.update_dir(d, parentname, link).await?;
        }
        Ok(link)
    }

    async fn dest_components(
        &mut self,
        dest: &DagfsDestination<S>,
    ) -> anyhow::Result<(Vec<(HostDirectoryFor<S>, Name)>, LinkFor<S>, Name)> {
        let mut components = vec![];
        let (root, intermediates, lastname) = dest.link_intermediates_and_last_name();

        let mut link = root;
        for i in 0..intermediates.len() {
            let d: HostDirectoryFor<S> = self.read(&link).await?;
            let name = &intermediates[i];
            let nextlink = d.get(name).cloned().ok_or_else(|| {
                let subpath = dest.prefix_path(i - 1);
                anyhow::anyhow!("link name {name:?} not found in {subpath}")
            })?;
            components.push((d, name.to_string()));
            link = nextlink;
        }
        Ok((components, link, lastname.to_string()))
    }

    async fn update_dir_link<T>(
        &mut self,
        dir: &LinkFor<S>,
        name: Name,
        object: T,
    ) -> anyhow::Result<LinkFor<S>>
    where
        T: ToDag<S>,
    {
        let hd = self.read(dir).await?;
        self.update_dir(hd, name, object).await
    }

    async fn update_dir<T>(
        &mut self,
        mut dir: HostDirectoryFor<S>,
        name: Name,
        object: T,
    ) -> anyhow::Result<LinkFor<S>>
    where
        T: ToDag<S>,
    {
        let link = self.commit(object).await?;
        dir.insert(name, link)?;
        self.commit(dir).await
    }
}
