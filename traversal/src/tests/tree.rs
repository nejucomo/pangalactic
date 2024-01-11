use std::convert::Infallible;

use crate::TraversableDag;

#[derive(Clone, Debug)]
pub struct Tree {
    pub id: u64,
    pub children: &'static [Tree],
}

impl Tree {
    pub const fn node(id: u64) -> Self {
        Tree { id, children: &[] }
    }
}

impl From<Tree> for u64 {
    fn from(t: Tree) -> Self {
        t.id
    }
}

impl TraversableDag for Tree {
    type Error = Infallible;
    type ChildStream = tokio_stream::Iter<std::vec::IntoIter<Result<Tree, Infallible>>>;
    type ChildrenFut = std::future::Ready<Result<Self::ChildStream, Infallible>>;

    fn children(&self) -> Self::ChildrenFut {
        std::future::ready(Ok(tokio_stream::iter(
            self.children
                .iter()
                .map(|t| Ok(t.clone()))
                // This is super goofy: we collect into a vec just so we have an easy-to-name type for ChildStream:
                .collect::<Vec<_>>()
                .into_iter(),
        )))
    }
}
