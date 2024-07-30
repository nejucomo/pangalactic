use crate::{Dagio, DagioCommit, DagioLoad};
use async_trait::async_trait;
use pangalactic_hostdir::HostDirectory;
use pangalactic_layer_cidmeta::CidMeta;
use pangalactic_link::Link;
use pangalactic_store::Store;

#[cfg_attr(not(doc), async_trait)]
impl<S> DagioCommit<S> for HostDirectory<S::CID>
where
    S: Store,
{
    async fn commit_into_dagio(
        self,
        dagio: &mut Dagio<S>,
    ) -> anyhow::Result<Link<CidMeta<S::CID>>> {
        use pangalactic_link::Link;
        use pangalactic_linkkind::LinkKind::Dir;
        use pangalactic_serialization::serialize;
        use tokio::io::AsyncWriteExt;

        let mut w = dagio.0.open_writer().await?;
        let buf = serialize(&self)?;
        w.write_all(&buf).await?;
        dagio
            .0
            .commit_writer(w)
            .await
            .map(|key| Link::new(Dir, key))
    }
}

#[cfg_attr(not(doc), async_trait)]
impl<const K: usize, S, N> DagioCommit<S> for [(N, Link<CidMeta<S::CID>>); K]
where
    S: Store,
    N: Send,
    String: From<N>,
{
    async fn commit_into_dagio(
        self,
        dagio: &mut Dagio<S>,
    ) -> anyhow::Result<Link<CidMeta<S::CID>>> {
        dagio
            .commit(HostDirectory::from_iter(self.into_iter()))
            .await
    }
}

#[cfg_attr(not(doc), async_trait)]
impl<S> DagioCommit<S> for tokio::fs::ReadDir
where
    S: Store,
{
    async fn commit_into_dagio(
        mut self,
        dagio: &mut Dagio<S>,
    ) -> anyhow::Result<Link<CidMeta<S::CID>>> {
        use anyhow_std::OsStrAnyhow;

        let mut hd = HostDirectory::default();
        while let Some(dirent) = self.next_entry().await? {
            let name = dirent.file_name().to_str_anyhow()?.to_string();
            let link = dagio.commit(dirent.path().as_path()).await?;
            hd.insert(name, link)?;
        }

        dagio.commit(hd).await
    }
}

#[cfg_attr(not(doc), async_trait)]
impl<S> DagioLoad<S> for HostDirectory<S::CID>
where
    S: Store,
{
    async fn load_from_dagio(
        dagio: &Dagio<S>,
        link: &Link<CidMeta<S::CID>>,
    ) -> anyhow::Result<Self> {
        use pangalactic_link::Link;
        use pangalactic_linkkind::LinkKind::{Dir, File};

        let cid = link.peek_cid_kind(Dir)?;
        let translink = Link::new(File, cid.clone());
        let bytes: Vec<u8> = dagio.load(&translink).await?;
        let dir = pangalactic_serialization::deserialize(bytes)?;
        Ok(dir)
    }
}
