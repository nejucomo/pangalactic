mod dagcase;
mod tree;

use self::dagcase::Case;
use self::tree::Tree;
use crate::TraversableDag;
use test_case::test_case;

// Cases:
const TREE_SINGLETON: Case<Tree, u64> = Case {
    dag: Tree::node(0),
    children: &[],
    bfs: &[0],
    dfs: &[0],
};

const TREE_V: Case<Tree, u64> = Case {
    dag: Tree {
        id: 0,
        children: &[Tree::node(1), Tree::node(2)],
    },
    children: &[1, 2],
    bfs: &[0, 1, 2],
    dfs: &[1, 2, 0],
};

const TREE_V_V_NODE: Case<Tree, u64> = Case {
    dag: Tree {
        id: 0,
        children: &[
            Tree {
                id: 1,
                children: &[Tree::node(2), Tree::node(3)],
            },
            Tree::node(4),
        ],
    },
    children: &[1, 4],
    bfs: &[0, 1, 4, 2, 3],
    dfs: &[2, 3, 1, 4, 0],
};

const TREE_V_NODE_V: Case<Tree, u64> = Case {
    dag: Tree {
        id: 0,
        children: &[
            Tree::node(1),
            Tree {
                id: 2,
                children: &[Tree::node(3), Tree::node(4)],
            },
        ],
    },
    children: &[1, 2],
    bfs: &[0, 1, 2, 3, 4],
    dfs: &[1, 3, 4, 2, 0],
};

#[test_case(TREE_SINGLETON)]
#[test_case(TREE_V)]
#[test_case(TREE_V_V_NODE)]
#[test_case(TREE_V_NODE_V)]
#[tokio::test]
async fn children<D, V>(case: Case<D, V>) -> anyhow::Result<()>
where
    D: TraversableDag,
    D::Error: std::error::Error + Send + Sync + 'static,
    V: PartialEq + From<D> + std::fmt::Debug + 'static,
{
    case.verify_children().await
}

#[test_case(TREE_SINGLETON)]
#[test_case(TREE_V)]
#[test_case(TREE_V_V_NODE)]
#[test_case(TREE_V_NODE_V)]
#[tokio::test]
async fn breadth_first_traversal<D, V>(case: Case<D, V>) -> anyhow::Result<()>
where
    D: TraversableDag,
    D::Error: std::error::Error + Send + Sync + 'static,
    V: PartialEq + From<D> + std::fmt::Debug + 'static,
{
    case.verify_breadth_first_traversal().await
}

#[test_case(TREE_SINGLETON)]
#[test_case(TREE_V)]
#[test_case(TREE_V_V_NODE)]
#[test_case(TREE_V_NODE_V)]
#[tokio::test]
async fn depth_first_traversal<D, V>(case: Case<D, V>) -> anyhow::Result<()>
where
    D: TraversableDag,
    D::Error: std::error::Error + Send + Sync + 'static,
    V: PartialEq + From<D> + std::fmt::Debug + 'static,
{
    case.verify_depth_first_traversal().await
}
