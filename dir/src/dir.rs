use std::collections::BTreeMap;
use std::marker::Unpin;
use tokio::io::AsyncWrite;

const SERIALIZATION_VERSION: u64 = 0;

#[derive(Debug, Default)]
pub struct Directory<L>(BTreeMap<Name, L>);

// TODO: newtype String which excludes illegal names:
pub type Name = String;

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

    pub async fn serialize_into<W>(self, mut w: W) -> anyhow::Result<()>
    where
        W: AsyncWrite + Unpin,
    {
        use crate::flexint::write_flexint;
        // use tokio::io::AsyncWriteExt;

        write_flexint(&mut w, SERIALIZATION_VERSION).await?;
        write_flexint(&mut w, self.0.len()).await?;

        todo!();
    }
}
