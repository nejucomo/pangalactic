use std::fmt;
use std::str::FromStr;

use pangalactic_chomper::Chomper;
use pangalactic_cid::ContentIdentifier;
use pangalactic_linkkind::LinkKind;
use pangalactic_serialization::b64;
use serde::de::DeserializeOwned;
use serde::{Deserialize, Serialize};

use crate::SCHEME;

#[derive(Clone, Debug, Eq, PartialEq, Deserialize, Serialize)]
pub struct Link<C> {
    kind: LinkKind,
    cid: C,
}

impl<C> ContentIdentifier for Link<C> where
    C: Clone + fmt::Debug + Eq + PartialEq + DeserializeOwned + Serialize + Send + Sync
{
}

impl<C> Link<C> {
    pub fn new(kind: LinkKind, cid: C) -> Self {
        Link { kind, cid }
    }

    pub fn kind(&self) -> LinkKind {
        self.kind
    }

    pub fn peek_cid(&self) -> &C {
        &self.cid
    }

    pub fn peek_cid_kind(&self, kind: LinkKind) -> anyhow::Result<&C> {
        if self.kind == kind {
            Ok(&self.cid)
        } else {
            Err(anyhow::Error::msg(format!(
                "expected link kind {:?}, found {:?}",
                kind, self.kind
            )))
        }
    }

    pub fn unwrap(self) -> (LinkKind, C) {
        (self.kind, self.cid)
    }

    fn from_str_without_context(s: &str) -> anyhow::Result<Self>
    where
        C: DeserializeOwned,
    {
        let mut chomper = Chomper::from(s);
        chomper.require_prefix(":", SCHEME)?;

        let kindstr = chomper.chomp_prefix("-")?;
        let kind = kindstr.parse()?;
        let cid = b64::deserialize(chomper)?;

        Ok(Link { kind, cid })
    }
}

impl<C> FromStr for Link<C>
where
    C: DeserializeOwned,
{
    type Err = anyhow::Error;

    fn from_str(s: &str) -> anyhow::Result<Self> {
        use anyhow::Context;

        Link::from_str_without_context(s).with_context(|| format!("while parsing Link {s:?}"))
    }
}

impl<C> fmt::Display for Link<C>
where
    C: Serialize,
{
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        let kind = self.kind;
        let cid = b64::serialize(&self.cid).map_err(|_| fmt::Error)?;
        write!(f, "{SCHEME}:{kind}-{cid}")
    }
}
