mod dagcase;
mod pathimpl;
mod tree;

use self::dagcase::{Case, DagCase};
use self::pathimpl::mkpv;
use self::tree::{
    MkTree::{Branch, Node},
    Tree,
};
use crate::TraversableDag;
use std::path::PathBuf;
use test_case::test_case;

// Cases:
const TREE_SINGLETON: Case<Tree> = Case {
    dag: Node,
    children: &[],
    bfs: &[0],
    dfs: &[0],
};

const TREE_V: Case<Tree> = Case {
    dag: Branch(&Node, &Node),
    children: &[1, 2],
    bfs: &[0, 1, 2],
    dfs: &[1, 2, 0],
};

const TREE_V_V_NODE: Case<Tree> = Case {
    dag: Branch(&Branch(&Node, &Node), &Node),
    children: &[1, 4],
    bfs: &[0, 1, 4, 2, 3],
    dfs: &[2, 3, 1, 4, 0],
};

const TREE_V_NODE_V: Case<Tree> = Case {
    dag: Branch(&Node, &Branch(&Node, &Node)),
    children: &[1, 2],
    bfs: &[0, 1, 2, 3, 4],
    dfs: &[1, 3, 4, 2, 0],
};

const PATHBUF_EMPTY: Case<PathBuf> = Case {
    dag: &[],
    children: &[],
    bfs: &[mkpv("")],
    dfs: &[mkpv("")],
};

const PATHBUF_TREE: Case<PathBuf> = Case {
    dag: &["foo", "bar/quz", "bar/wux"],
    children: &[mkpv("bar"), mkpv("foo")],
    bfs: &[
        mkpv(""),
        mkpv("bar"),
        mkpv("foo"),
        mkpv("bar/quz"),
        mkpv("bar/wux"),
    ],
    dfs: &[
        mkpv("bar/quz"),
        mkpv("bar/wux"),
        mkpv("bar"),
        mkpv("foo"),
        mkpv(""),
    ],
};

#[test_case(TREE_SINGLETON)]
#[test_case(TREE_V)]
#[test_case(TREE_V_V_NODE)]
#[test_case(TREE_V_NODE_V)]
#[test_case(PATHBUF_EMPTY)]
#[test_case(PATHBUF_TREE)]
#[tokio::test]
async fn children<D>(case: Case<D>) -> anyhow::Result<()>
where
    D: DagCase,
    <D as TraversableDag>::Error: Send + Sync + 'static,
{
    case.verify_children().await
}

#[test_case(TREE_SINGLETON)]
#[test_case(TREE_V)]
#[test_case(TREE_V_V_NODE)]
#[test_case(TREE_V_NODE_V)]
#[test_case(PATHBUF_EMPTY)]
#[test_case(PATHBUF_TREE)]
#[tokio::test]
async fn breadth_first_traversal<D>(case: Case<D>) -> anyhow::Result<()>
where
    D: DagCase,
    <D as TraversableDag>::Error: Send + Sync + 'static,
{
    case.verify_breadth_first_traversal().await
}

#[test_case(TREE_SINGLETON)]
#[test_case(TREE_V)]
#[test_case(TREE_V_V_NODE)]
#[test_case(TREE_V_NODE_V)]
#[test_case(PATHBUF_EMPTY)]
#[test_case(PATHBUF_TREE)]
#[tokio::test]
async fn depth_first_traversal<D>(case: Case<D>) -> anyhow::Result<()>
where
    D: DagCase,
    <D as TraversableDag>::Error: Send + Sync + 'static,
{
    case.verify_depth_first_traversal().await
}
