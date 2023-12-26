use pangalactic_dagio::{Dagio, FromDag, HostDirectoryFor, LinkFor, ToDag};
use pangalactic_dir::Name;
use pangalactic_layer_cidmeta::CidMetaLayer;
use pangalactic_store::Store;
use pangalactic_storepath::StorePath;

#[derive(Debug, Default, derive_more::Deref, derive_more::DerefMut)]
pub struct Dagfs<S>(Dagio<S>)
where
    S: Store;

pub type DagfsPath<S> = StorePath<<CidMetaLayer<S> as Store>::Cid>;

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
    /// Read the object at `source`
    pub async fn read_path<T>(&mut self, source: &DagfsPath<S>) -> anyhow::Result<T>
    where
        T: FromDag<S>,
    {
        let (root, names) = source.link_and_path_slice();
        let mut link = root;
        for i in 0..names.len() {
            let d: HostDirectoryFor<S> = self.read(&link).await?;
            let name = &names[i];
            link = d.get(name).cloned().ok_or_else(|| {
                let subpath = source.prefix_path(i - 1);
                anyhow::anyhow!("link name {name:?} not found in {subpath}")
            })?;
        }
        self.read(&link).await
    }

    /// Set `dest` to point to `object`
    ///
    /// The final component of `dest` is the name to associate to `object`. It is an error for `dest` to not have any name components (i.e. be a link without any path suffix.
    ///
    /// The returned link points to a new root with equivalent tree structure as `dest`, with the new `object` linked at the new path.
    pub async fn commit_path<T>(
        &mut self,
        dest: &DagfsPath<S>,
        object: T,
    ) -> anyhow::Result<LinkFor<S>>
    where
        T: ToDag<S>,
    {
        // Scan down to the penultimate path component, saving tree structure and name path
        let (root, names) = dest.link_and_path_slice();
        let mut names: Vec<Name> = names.iter().cloned().collect();
        let setname = names.pop().ok_or_else(|| {
            anyhow::anyhow!("destination {dest} has no final path component name")
        })?;
        let mut entries = vec![];
        let mut link = root;
        for i in 0..names.len() {
            let d: HostDirectoryFor<S> = self.read(&link).await?;
            let name = &names[i];
            let nextlink = d.get(name).cloned().ok_or_else(|| {
                let subpath = dest.prefix_path(i - 1);
                anyhow::anyhow!("link name {name:?} not found in {subpath}")
            })?;
            entries.push((d, name.to_string()));
            link = nextlink;
        }

        // Rebuild the structure with the new object:
        let mut d: HostDirectoryFor<S> = self.read(&link).await?;
        let mut link = self.commit(object).await?;
        d.insert(setname, link)?;

        link = self.commit(d).await?;
        for (mut d, parentname) in entries.into_iter().rev() {
            d.insert(parentname, link)?;
            link = self.commit(d).await?;
        }
        Ok(link)
    }
}
