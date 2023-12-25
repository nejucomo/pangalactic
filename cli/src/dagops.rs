use std::path::{Path, PathBuf};

use either::Either;
use pangalactic_dagio::Dagio;
use pangalactic_layer_cidmeta::CidMeta;
use pangalactic_link::Link;
use pangalactic_path::AnyPath;
use pangalactic_store_dirdb::DirDbStore;

#[derive(Debug, Default)]
pub struct DagOps(Dagio<DirDbStore>);

pub type AnyPathDo = AnyPath<CidMeta<DirDbStore>>;
pub type LinkDo = Link<CidMeta<DirDbStore>>;

impl DagOps {
    pub async fn store_file_put(&mut self) -> anyhow::Result<()> {
        let mut r = tokio::io::stdin();
        let mut w = self.0.open_file_writer().await?;
        tokio::io::copy(&mut r, &mut w).await?;
        let link = self.0.commit_file_writer(w).await?;
        println!("{link}");
        Ok(())
    }

    pub async fn store_file_get(&mut self, link: &LinkDo) -> anyhow::Result<()> {
        let mut r = self.0.open_file_reader(link).await?;
        let mut w = tokio::io::stdout();
        tokio::io::copy(&mut r, &mut w).await?;
        Ok(())
    }

    pub async fn store_dir_empty(&mut self) -> anyhow::Result<()> {
        use pangalactic_dagio::{HostDirectoryFor, ToDag};

        let hd = HostDirectoryFor::default();
        let link = hd.into_dag(&mut self.0).await?;
        println!("{link}");
        Ok(())
    }

    pub async fn store_dir_link(
        &mut self,
        dir: &LinkDo,
        name: &str,
        target: &LinkDo,
    ) -> anyhow::Result<()> {
        use pangalactic_dagio::{FromDag, HostDirectoryFor, ToDag};

        let mut hd = HostDirectoryFor::from_dag(&mut self.0, dir).await?;
        hd.insert(name.to_string(), target.clone())?;
        let link = hd.into_dag(&mut self.0).await?;
        println!("{link}");
        Ok(())
    }

    pub async fn store_dir_unlink(&mut self, dir: &LinkDo, name: &str) -> anyhow::Result<()> {
        use pangalactic_dagio::{FromDag, HostDirectoryFor, ToDag};

        let mut hd = HostDirectoryFor::from_dag(&mut self.0, dir).await?;
        hd.remove_required(name)?;
        let link = hd.into_dag(&mut self.0).await?;
        println!("{link}");
        Ok(())
    }

    pub async fn store_dir_list(&mut self, dir: &LinkDo) -> anyhow::Result<()> {
        use pangalactic_dagio::{FromDag, HostDirectoryFor};

        let hd = HostDirectoryFor::from_dag(&mut self.0, dir).await?;
        for (name, link) in hd {
            println!("{link} {name}");
        }
        Ok(())
    }

    pub async fn store_tree_manifest(&mut self, root: &LinkDo) -> anyhow::Result<()> {
        todo!("manifest {root}");
    }

    pub async fn store_tree_import(&mut self, source: &Path) -> anyhow::Result<()> {
        todo!("import {:?}", source.display());
    }

    pub async fn store_tree_export(&mut self, root: &LinkDo, dest: &Path) -> anyhow::Result<()> {
        todo!("export {root} -> {:?}", dest.display());
    }

    pub async fn store_copy(&mut self, source: AnyPathDo, dest: AnyPathDo) -> anyhow::Result<()> {
        use AnyPath::*;
        use Either::*;

        let source = self.resolve(source).await?;
        match (source, dest) {
            (Left(hostsrc), Host(hostdst)) => {
                tokio::fs::copy(hostsrc, hostdst).await?;
                Ok(())
            }
            (Left(hostsrc), Store(storedst)) => todo!("import {hostsrc:?} -> {storedst:?}"),
            (Right(storesrc), Host(hostdst)) => todo!("export {storesrc:?} -> {hostdst:?}"),
            (Right(storesrc), Store(storedst)) => todo!("store copy {storesrc:?} -> {storedst:?}"),
        }
    }

    async fn resolve(&mut self, path: AnyPathDo) -> anyhow::Result<Either<PathBuf, LinkDo>> {
        use either::Either::*;
        use pangalactic_dagio::{FromDag, HostDirectoryFor};
        use AnyPath::*;

        match path {
            Host(path) => Ok(Left(path)),
            Store(storepath) => {
                let (mut link, pathparts) = storepath.into();
                for part in pathparts {
                    let mut hd = HostDirectoryFor::from_dag(&mut self.0, &link).await?;
                    link = hd.remove_required(&part)?;
                }
                Ok(Right(link))
            }
        }
    }
}
