use crate::Link;
use async_trait::async_trait;
use dagwasm_serialization::{AsyncDeserialize, AsyncSerialize};
use std::collections::BTreeMap;
use std::marker::Unpin;
use tokio::io::{AsyncRead, AsyncWrite};

const SERIALIZATION_VERSION: u64 = 0;

#[derive(Debug, PartialEq, Eq)]
pub struct Directory<K>(BTreeMap<Name, Link<K>>);

// TODO: newtype String which excludes illegal names:
pub type Name = String;

impl<K> Default for Directory<K> {
    fn default() -> Self {
        Directory(BTreeMap::default())
    }
}

impl<K> Directory<K> {
    pub fn insert(&mut self, name: Name, link: Link<K>) -> anyhow::Result<()> {
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
impl<K> AsyncSerialize for Directory<K>
where
    K: AsyncSerialize + Send + Sync,
{
    async fn write_into<W>(&self, mut w: W) -> anyhow::Result<()>
    where
        W: AsyncWrite + Unpin + Send,
    {
        SERIALIZATION_VERSION.write_into(&mut w).await?;
        self.0.len().write_into(&mut w).await?;

        // We want to do this, but then links don't live long enough for async `write_into`:
        // for (name, link) in self.0.iter() {

        let entries: Vec<(&'_ String, &'_ Link<K>)> = self.0.iter().collect();
        for (name, link) in entries {
            name.write_into(&mut w).await?;
            link.write_into(&mut w).await?;
        }

        Ok(())
    }
}

#[async_trait]
impl<K> AsyncDeserialize for Directory<K>
where
    K: AsyncDeserialize + Send + Sync,
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
            let link = Link::<K>::read_from(&mut r).await?;
            d.insert(name, link)?;
        }

        Ok(d)
    }
}
