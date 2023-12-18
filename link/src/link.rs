use pangalactic_linkkind::LinkKind;
use std::fmt::Debug;

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
        K: Debug,
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
