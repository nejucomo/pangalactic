use dagwasm_blobstore::BlobStore;
use std::fmt::Debug;

#[derive(Debug)]
pub enum LinkKind {
    File,
    Dir,
}

#[derive(Debug)]
pub enum Link<K> {
    File(K),
    Dir(K),
}

pub type LinkFor<BS> = Link<<BS as BlobStore>::Key>;

impl<K> Link<K> {
    pub fn unwrap_key(self, kind: LinkKind) -> anyhow::Result<K>
    where
        K: Debug,
    {
        use Link::*;
        use LinkKind as Kind;

        match (kind, self) {
            (Kind::File, File(k)) => Ok(k),
            (Kind::Dir, Dir(k)) => Ok(k),
            (expected, found) => Err(anyhow::Error::msg(format!(
                "expected link kind {:?}, found {:?}",
                expected, found
            ))),
        }
    }
}
