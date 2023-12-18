use pangalactic_linkkind::LinkKind;
use std::fmt;
use std::str::FromStr;

#[derive(Clone, Debug, PartialEq, Eq)]
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
}

impl<K> FromStr for Link<K>
where
    K: FromStr<Err = anyhow::Error>,
{
    type Err = anyhow::Error;

    fn from_str(s: &str) -> anyhow::Result<Self> {
        let (kindtext, suffix) = s
            .split_once(':')
            .ok_or_else(|| anyhow::anyhow!("missing ':'"))?;
        let kind = kindtext.parse()?;
        let key = suffix.parse()?;
        Ok(Link::new(kind, key))
    }
}

impl<K> fmt::Display for Link<K>
where
    K: fmt::Display,
{
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        self.kind.fmt(f)?;
        ':'.fmt(f)?;
        self.key.fmt(f)?;
        Ok(())
    }
}
