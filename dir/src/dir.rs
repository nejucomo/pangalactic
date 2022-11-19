use crate::codec::{AsyncDeserialize, AsyncSerialize};
use async_trait::async_trait;
use std::collections::BTreeMap;
use std::marker::Unpin;
use tokio::io::{AsyncRead, AsyncWrite};

const SERIALIZATION_VERSION: u64 = 0;

#[derive(Debug)]
pub struct Directory<L>(BTreeMap<Name, L>);

// TODO: newtype String which excludes illegal names:
pub type Name = String;

impl<L> Default for Directory<L> {
    fn default() -> Self {
        Directory(BTreeMap::default())
    }
}

impl<L> Directory<L> {
    pub fn insert(&mut self, name: Name, link: L) -> anyhow::Result<()> {
        let errname = name.clone();
        if self.0.insert(name, link).is_none() {
            Ok(())
        } else {
            Err(anyhow::Error::msg(format!(
                "duplicate entry for {:?}",
                errname
            )))
        }
    }
}

#[async_trait]
impl<L> AsyncSerialize for Directory<L>
where
    L: AsyncSerialize + Send + Sync,
{
    async fn write_into<W>(&self, mut w: W) -> anyhow::Result<()>
    where
        W: AsyncWrite + Unpin + Send,
    {
        SERIALIZATION_VERSION.write_into(&mut w).await?;
        self.0.len().write_into(&mut w).await?;

        // We want to do this, but then `L` doesn't live long enough:
        // for (name, link) in self.0.iter() {

        let entries: Vec<(&'_ String, &'_ L)> = self.0.iter().collect();
        for (name, link) in entries {
            name.write_into(&mut w).await?;
            link.write_into(&mut w).await?;
        }

        Ok(())
    }
}

#[async_trait]
impl<L> AsyncDeserialize for Directory<L>
where
    L: AsyncDeserialize + Send + Sync,
{
    async fn read_from<R>(mut r: R) -> anyhow::Result<Self>
    where
        R: AsyncRead + Unpin + Send,
    {
        let version = u64::read_from(&mut r).await?;
        if version != SERIALIZATION_VERSION {
            return Err(anyhow::Error::msg(format!(
                "expected serialization version {}, found {}",
                SERIALIZATION_VERSION, version
            )));
        }

        let mut d = Directory::default();
        let entrycount = usize::read_from(&mut r).await?;
        for _ in 0..entrycount {
            let name = Name::read_from(&mut r).await?;
            let link = L::read_from(&mut r).await?;
            d.insert(name, link)?;
        }

        Ok(d)
    }
}
