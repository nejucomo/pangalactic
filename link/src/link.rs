use pangalactic_linkkind::LinkKind;
use pangalactic_store::Store;
use serde::{Deserialize, Serialize};
use std::fmt;
use std::str::FromStr;

#[derive(Debug, Eq, Deserialize, Serialize)]
pub struct Link<S>
where
    S: Store,
{
    kind: LinkKind,
    key: S::CID,
}

impl<S> Clone for Link<S>
where
    S: Store,
{
    fn clone(&self) -> Self {
        Link {
            kind: self.kind,
            key: self.key.clone(),
        }
    }
}

impl<S> PartialEq for Link<S>
where
    S: Store,
{
    fn eq(&self, other: &Self) -> bool {
        (self.kind == other.kind) && (self.key == other.key)
    }
}

impl<S> Link<S>
where
    S: Store,
{
    // TODO: const_format
    pub fn prefix() -> String {
        format!("{}://", S::SCHEME)
    }

    pub fn new(kind: LinkKind, key: S::CID) -> Self {
        Link { kind, key }
    }

    pub fn kind(&self) -> LinkKind {
        self.kind
    }

    pub fn peek_key(&self) -> &S::CID {
        &self.key
    }

    pub fn peek_key_kind(&self, kind: LinkKind) -> anyhow::Result<&S::CID> {
        if self.kind == kind {
            Ok(&self.key)
        } else {
            Err(anyhow::Error::msg(format!(
                "expected link kind {:?}, found {:?}",
                kind, self.kind
            )))
        }
    }

    pub fn unwrap(self) -> (LinkKind, S::CID) {
        (self.kind, self.key)
    }

    fn from_str_without_context(s: &str) -> anyhow::Result<Self> {
        let prefix = Self::prefix();
        let linkstr = s
            .strip_prefix(&prefix)
            .ok_or_else(|| anyhow::anyhow!("missing expected prefix {prefix:?}"))?;

        let (kindstr, keystr) = linkstr
            .split_once('-')
            .ok_or_else(|| anyhow::anyhow!("expected `<KIND>-<KEY>` encoding"))?;

        let kind = kindstr.parse()?;
        let key = keystr.parse()?;

        Ok(Link { kind, key })
    }
}

impl<S> FromStr for Link<S>
where
    S: Store,
{
    type Err = anyhow::Error;

    fn from_str(s: &str) -> anyhow::Result<Self> {
        use anyhow::Context;

        Link::from_str_without_context(s).with_context(|| format!("while parsing Link {s:?}"))
    }
}

impl<S> fmt::Display for Link<S>
where
    S: Store,
{
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "{}{}-{}", Self::prefix(), self.kind, self.key)
    }
}
