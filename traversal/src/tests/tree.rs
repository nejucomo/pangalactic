use std::convert::Infallible;

use crate::TraversableDag;

#[derive(Clone, Debug)]
pub struct Tree {
    pub id: u64,
    children: Vec<Tree>,
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

pub trait TreeBuilder: Sized + std::fmt::Debug {
    fn build_tree(self) -> Tree {
        self.build_tree_with_idgen(&mut IdGen(0))
    }

    fn build_tree_with_idgen(self, idgen: &mut IdGen) -> Tree;
}

pub struct IdGen(u64);

impl IdGen {
    fn next(&mut self) -> u64 {
        let id = self.0;
        self.0 += 1;
        id
    }
}

#[derive(Debug)]
pub struct NodeMaker;

impl TreeBuilder for NodeMaker {
    fn build_tree_with_idgen(self, idgen: &mut IdGen) -> Tree {
        Tree {
            id: idgen.next(),
            children: vec![],
        }
    }
}

impl<A, B> TreeBuilder for (A, B)
where
    A: TreeBuilder,
    B: TreeBuilder,
{
    fn build_tree_with_idgen(self, idgen: &mut IdGen) -> Tree {
        let (a, b) = self;
        Tree {
            id: idgen.next(),
            children: vec![
                a.build_tree_with_idgen(idgen),
                b.build_tree_with_idgen(idgen),
            ],
        }
    }
}
