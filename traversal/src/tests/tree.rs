use crate::tests::dagcase::DagCase;
use crate::TraversableDag;
use async_trait::async_trait;

#[derive(Clone, Debug)]
pub struct Tree {
    pub id: u64,
    pub children: Vec<Tree>,
}

impl From<Tree> for u64 {
    fn from(t: Tree) -> Self {
        t.id
    }
}

#[derive(Debug)]
pub enum MkTree {
    Node,
    Branch(&'static MkTree, &'static MkTree),
}

#[async_trait]
impl DagCase for Tree {
    type Ctr = MkTree;
    type Verifier = u64;

    async fn setup(mt: MkTree) -> anyhow::Result<Tree> {
        Ok(mt.build(&mut 0))
    }
}

impl TraversableDag for Tree {
    type Error = anyhow::Error;
    type ChildStream = tokio_stream::Iter<std::vec::IntoIter<Result<Tree, anyhow::Error>>>;
    type ChildrenFut = std::future::Ready<Result<Self::ChildStream, anyhow::Error>>;

    fn children(&self) -> Self::ChildrenFut {
        std::future::ready(Ok(tokio_stream::iter(
            self.children
                .iter()
                .map(|t| Ok(t.clone()))
                // This is super goofy: we collect into a vec just so we have an easy-to-name type for ChildStream, but it shouldn't matter because all of the test cases are tiny:
                .collect::<Vec<_>>()
                .into_iter(),
        )))
    }
}

impl MkTree {
    fn build(&self, ids: &mut u64) -> Tree {
        use MkTree::*;

        let id = *ids;
        *ids += 1;

        Tree {
            id,
            children: match self {
                Node => vec![],
                Branch(a, b) => vec![a.build(ids), b.build(ids)],
            },
        }
    }
}
