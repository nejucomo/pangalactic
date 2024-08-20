//! Static seed directory structure

use anyhow::Result;
// use include_dir::{include_dir, Dir};
use pangalactic_layer_dir::LinkDirectoryLayer;
use pangalactic_store::{Commit, Store};

pub(crate) struct StaticDir;

impl<S> Commit<LinkDirectoryLayer<S>> for StaticDir
where
    S: Store,
{
    async fn commit_into_store(
        self,
        _store: &mut LinkDirectoryLayer<S>,
    ) -> Result<<LinkDirectoryLayer<S> as Store>::CID> {
        todo!()
    }
}

// static STATIC_DIR: Dir<'_> = include_dir!("$CARGO_MANIFEST_DIR/staticdir/");
