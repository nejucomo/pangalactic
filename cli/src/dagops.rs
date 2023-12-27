use anyhow_std::{OsStrAnyhow, PathAnyhow};
use either::Either;
use pangalactic_dagfs::Dagfs;
use pangalactic_dagio::{FromDag, HostDirectoryFor, ToDag};
use pangalactic_dir::Name;
use pangalactic_layer_cidmeta::CidMeta;
use pangalactic_link::Link;
use pangalactic_store_dirdb::DirDbStore;
use pangalactic_storepath::StorePath;
use std::path::{Path, PathBuf};

#[derive(Debug, Default)]
pub struct DagOps(Dagfs<DirDbStore>);

pub type DagfsDo = Dagfs<DirDbStore>;
pub type DirectoryDo = HostDirectoryFor<DirDbStore>;
pub type LinkDo = Link<CidMeta<DirDbStore>>;
pub type StorePathDo = StorePath<CidMeta<DirDbStore>>;

impl DagOps {
    pub async fn store_file_put(&mut self) -> anyhow::Result<()> {
        let link = self.0.commit_file_from_reader(tokio::io::stdin()).await?;
        println!("{link}");
        Ok(())
    }

    pub async fn store_file_get(&mut self, link: &LinkDo) -> anyhow::Result<()> {
        self.0
            .read_file_into_writer(link, tokio::io::stdout())
            .await?;
        Ok(())
    }

    pub async fn store_dir_empty(&mut self) -> anyhow::Result<()> {
        let hd = DirectoryDo::default();
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
        let mut hd = HostDirectoryFor::from_dag(&mut self.0, dir).await?;
        hd.insert(name.to_string(), target.clone())?;
        let link = hd.into_dag(&mut self.0).await?;
        println!("{link}");
        Ok(())
    }

    pub async fn store_dir_unlink(&mut self, dir: &LinkDo, name: &str) -> anyhow::Result<()> {
        let mut hd = HostDirectoryFor::from_dag(&mut self.0, dir).await?;
        hd.remove_required(name)?;
        let link = hd.into_dag(&mut self.0).await?;
        println!("{link}");
        Ok(())
    }

    pub async fn store_dir_list(&mut self, dir: &LinkDo) -> anyhow::Result<()> {
        let hd = HostDirectoryFor::from_dag(&mut self.0, dir).await?;
        for (name, link) in hd {
            println!("{link} {name}");
        }
        Ok(())
    }

    pub async fn store_tree_manifest(&mut self, root: &LinkDo) -> anyhow::Result<()> {
        use pangalactic_linkkind::LinkKind::*;

        println!("Manifest of {root}:");
        let mut stack: Vec<(LinkDo, String)> = vec![(root.clone(), "".to_string())];
        // Gather cells of rows independently for column alignment:
        let mut rows = vec![];

        while let Some((link, path)) = stack.pop() {
            match link.kind() {
                File => {
                    let size = link.peek_key().node_size();
                    rows.push((link.to_string(), size.to_string(), path));
                }
                Dir => {
                    let ddo = DirectoryDo::from_dag(&mut self.0, &link).await?;
                    for (childname, childlink) in ddo {
                        let childpath = format!("{}/{}", path, childname);
                        stack.push((childlink, childpath));
                    }
                }
            }
        }

        // Calculate column sizes:
        let mut width_link = 0;
        let mut width_size = 0;
        for (link, size, _) in rows.iter() {
            width_link = std::cmp::max(width_link, link.len());
            width_size = std::cmp::max(width_size, size.len());
        }

        // Sort:
        rows.sort_by(|(_, _, a), (_, _, b)| a.cmp(b));

        // Print column-aligned table:
        for (link, size, path) in rows {
            println!("{link:width_link$} {size:>width_size$} {path}");
        }
        Ok(())
    }

    pub async fn store_tree_import(&mut self, source: &Path) -> anyhow::Result<()> {
        let root = self.import_path(source).await?;
        println!("{root}");
        Ok(())
    }

    /// A stack-based (non-recursive) depth-first import routine
    ///
    /// Stack-based ensures resource usage does not bottleneck on the
    /// call stack for large filesystems, as well as working with standard
    /// async routines (rather than using `async-recursion` trait or other
    /// work-arounds).
    async fn import_path(&mut self, p: &Path) -> anyhow::Result<LinkDo> {
        use either::Either::*;

        let mut stack: Vec<(Name, DirectoryDo, Vec<PathBuf>)> = vec![];

        match self.import_process_path(p).await? {
            Left((_, link)) => {
                return Ok(link);
            }
            Right((name, children)) => {
                stack.push((name, DirectoryDo::default(), children));
            }
        };

        loop {
            let (_, ddo, children) = stack.last_mut().expect("internal loop invariant failure");

            if let Some(childpath) = children.pop() {
                // Make progress on linking the current directory's children:
                match self.import_process_path(&childpath).await? {
                    Left((childname, link)) => ddo.insert(childname, link)?,
                    Right((childname, gkids)) => {
                        // The current child is a directory, so recurse-equivalent by pushing onto the stack:
                        stack.push((childname, DirectoryDo::default(), gkids))
                    }
                }
            } else {
                let (name, ddo, _empty) = stack.pop().expect("internal loop invariant failure");
                let link = ddo.into_dag(&mut self.0).await?;
                if let Some((_, ddo, _)) = stack.last_mut() {
                    ddo.insert(name, link)?;
                } else {
                    return Ok(link);
                }
            }
        }
    }

    async fn import_process_path(
        &mut self,
        p: &Path,
    ) -> anyhow::Result<Either<(Name, LinkDo), (Name, Vec<PathBuf>)>> {
        use either::Either::*;

        let name = p.file_name_anyhow()?.to_str_anyhow()?.to_string();

        if p.is_file() {
            let f = tokio::fs::File::open(p).await?;
            let link = self.0.commit_file_from_reader(f).await?;
            Ok(Left((name, link)))
        } else if p.is_dir() {
            let mut children = vec![];
            let mut rd = tokio::fs::read_dir(p).await?;
            while let Some(dirent) = rd.next_entry().await? {
                children.push(dirent.path());
            }
            Ok(Right((name, children)))
        } else {
            anyhow::bail!("path is neither file nor dir: {:?}", p.display());
        }
    }

    pub async fn store_tree_export(&mut self, root: &LinkDo, dest: &Path) -> anyhow::Result<()> {
        use pangalactic_linkkind::LinkKind::*;
        use std::collections::VecDeque;

        // Breadth-first export:
        let mut queue = VecDeque::from([(root.clone(), dest.to_path_buf())]);
        while let Some((link, path)) = queue.pop_front() {
            match link.kind() {
                File => {
                    let f = tokio::fs::File::create(path).await?;
                    self.0.read_file_into_writer(&link, f).await?;
                }
                Dir => {
                    tokio::fs::create_dir(&path).await?;
                    let ddo = DirectoryDo::from_dag(&mut self.0, &link).await?;
                    for (name, childlink) in ddo {
                        let childpath = path.join(name);
                        queue.push_back((childlink, childpath));
                    }
                }
            }
        }

        Ok(())
    }

    pub async fn store_copy(
        &mut self,
        source: StorePathDo,
        dest: StorePathDo,
    ) -> anyhow::Result<()> {
        use Either::*;

        let source = self.resolve(source).await?;
        match source {
            Left(hostsrc) => todo!("import {hostsrc:?} -> {dest:?}"),
            Right(storesrc) => todo!("store copy {storesrc:?} -> {dest:?}"),
        }
    }

    async fn resolve(&mut self, path: StorePathDo) -> anyhow::Result<Either<PathBuf, LinkDo>> {
        use either::Either::*;

        let (mut link, pathparts) = path.into();
        for part in pathparts {
            let mut hd = HostDirectoryFor::from_dag(&mut self.0, &link).await?;
            link = hd.remove_required(&part)?;
        }
        Ok(Right(link))
    }
}
