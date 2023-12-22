use pangalactic_b64 as b64;
use pangalactic_linkkind::LinkKind;
use serde::{Deserialize, Serialize};
use std::fmt;
use std::str::FromStr;

#[derive(Clone, Debug, PartialEq, Eq, Deserialize, Serialize)]
pub struct Link<K> {
    kind: LinkKind,
    key: K,
}

impl<K> Link<K> {
    pub fn new(kind: LinkKind, key: K) -> Self {
        Link { kind, key }
    }

    pub fn kind(&self) -> LinkKind {
        self.kind
    }

    pub fn peek_key(&self) -> &K {
        &self.key
    }

    pub fn peek_key_kind(&self, kind: LinkKind) -> anyhow::Result<&K>
    where
        K: fmt::Debug,
    {
        if self.kind == kind {
            Ok(&self.key)
        } else {
            Err(anyhow::Error::msg(format!(
                "expected link kind {:?}, found {:?}",
                kind, self.kind
            )))
        }
    }

    pub fn unwrap(self) -> (LinkKind, K) {
        (self.kind, self.key)
    }

    fn from_str_without_context(s: &str) -> anyhow::Result<Self>
    where
        K: serde::de::DeserializeOwned,
    {
        use pangalactic_serialization::deserialize;

        let (kindtext, linkb64) = s
            .split_once('-')
            .ok_or_else(|| anyhow::anyhow!("missing '-'"))?;

        let kind: LinkKind = kindtext.parse()?;
        let bytes = b64::decode(linkb64)?;
        let key: K = deserialize(&bytes)?;

        Ok(Link::new(kind, key))
    }
}

impl<K> FromStr for Link<K>
where
    K: serde::de::DeserializeOwned,
{
    type Err = anyhow::Error;

    fn from_str(s: &str) -> anyhow::Result<Self> {
        use anyhow::Context;

        Link::from_str_without_context(s).with_context(|| format!("while parsing Link {s:?}"))
    }
}

impl<K> fmt::Display for Link<K>
where
    K: Serialize,
{
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        use pangalactic_serialization::serialize;

        self.kind.fmt(f)?;
        '-'.fmt(f)?;

        let bytes = serialize(&self.key).map_err(|_| std::fmt::Error)?;
        let s = b64::encode(bytes);
        s.fmt(f)
    }
}
