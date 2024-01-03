use pangalactic_store::{Store, StoreCid};
use serde::{Deserialize, Serialize};

#[derive(Debug, Deserialize, Serialize)]
pub struct CidMeta<S>
where
    S: Store,
{
    #[serde(bound(deserialize = "S:", serialize = "S:"))]
    pub(crate) cid: <S as Store>::CID,
    pub(crate) node_size: u64,
}

impl<S> CidMeta<S>
where
    S: Store,
{
    pub fn node_size(&self) -> u64 {
        self.node_size
    }
}

impl<S> StoreCid for CidMeta<S>
where
    S: Store,
{
    fn encode_fields(&self, dest: &mut Vec<String>) {
        use pangalactic_serialization::b64;

        dest.push(b64::serialize(&self.node_size).unwrap());
        self.cid.encode_fields(dest);
    }

    fn parse_fields<'a, I>(mut fields: I) -> anyhow::Result<Self>
    where
        I: Iterator<Item = &'a str>,
    {
        use pangalactic_serialization::b64;

        let node_size_field = fields
            .next()
            .ok_or_else(|| anyhow::anyhow!("missing node size field"))?;
        let node_size = b64::deserialize(node_size_field)?;
        let cid = S::CID::parse_fields(fields)?;
        Ok(CidMeta { cid, node_size })
    }
}

impl<S> Clone for CidMeta<S>
where
    S: Store,
{
    fn clone(&self) -> Self {
        CidMeta {
            cid: self.cid.clone(),
            node_size: self.node_size,
        }
    }
}

impl<S> PartialEq for CidMeta<S>
where
    S: Store,
{
    fn eq(&self, other: &Self) -> bool {
        (self.cid == other.cid) && (self.node_size == other.node_size)
    }
}

impl<S> Eq for CidMeta<S> where S: Store {}
