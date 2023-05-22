use crate::{Directory, Name};
use async_trait::async_trait;
use pangalactic_link::Link;
use pangalactic_serialization::{AsyncDeserialize, AsyncSerialize};
use std::marker::Unpin;
use tokio::io::{AsyncRead, AsyncWrite};

const SERIALIZATION_VERSION: u64 = 0;

pub type HostDirectory<K> = Directory<Link<K>>;

#[async_trait]
impl<K> AsyncSerialize for HostDirectory<K>
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
impl<K> AsyncDeserialize for HostDirectory<K>
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
                "expected serialization version {SERIALIZATION_VERSION}, found {version}"
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
