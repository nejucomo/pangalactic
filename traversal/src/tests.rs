mod tree;

use self::tree::{NodeMaker as N, Tree, TreeBuilder};
use crate::TraversableDag;
use std::marker::PhantomData;
use test_case::test_case;

// Tree cases:
#[derive(Debug)]
struct TreeCase<T> {
    phantom: PhantomData<T>,
    children: &'static [u64],
    bfs: &'static [u64],
    dfs: &'static [u64],
}

const SINGLETON: TreeCase<N> = TreeCase {
    phantom: PhantomData,
    children: &[],
    bfs: &[0],
    dfs: &[0],
};

const V: TreeCase<(N, N)> = TreeCase {
    phantom: PhantomData,
    children: &[1, 2],
    bfs: &[0, 1, 2],
    dfs: &[1, 2, 0],
};

const V_V_NODE: TreeCase<((N, N), N)> = TreeCase {
    phantom: PhantomData,
    children: &[1, 4],
    bfs: &[0, 1, 4, 2, 3],
    dfs: &[2, 3, 1, 4, 0],
};

const V_NODE_V: TreeCase<(N, (N, N))> = TreeCase {
    phantom: PhantomData,
    children: &[1, 2],
    bfs: &[0, 1, 2, 3, 4],
    dfs: &[1, 3, 4, 2, 0],
};

#[test_case(SINGLETON)]
#[test_case(V)]
#[test_case(V_V_NODE)]
#[test_case(V_NODE_V)]
#[tokio::test]
async fn children<T>(treecase: TreeCase<T>)
where
    T: TreeBuilder,
{
    let actual = stream_to_ids(dbg_build_tree::<T>().children().await.unwrap()).await;
    assert_eq!(actual, treecase.children);
}

#[test_case(SINGLETON)]
#[test_case(V)]
#[test_case(V_V_NODE)]
#[test_case(V_NODE_V)]
#[tokio::test]
async fn bfs<T>(treecase: TreeCase<T>)
where
    T: TreeBuilder,
{
    let actual = stream_to_ids(dbg_build_tree::<T>().traverse_breadth_first()).await;
    assert_eq!(actual, treecase.bfs);
}

#[test_case(SINGLETON)]
#[test_case(V)]
#[test_case(V_V_NODE)]
#[test_case(V_NODE_V)]
#[tokio::test]
async fn dfs<T>(treecase: TreeCase<T>)
where
    T: TreeBuilder,
{
    let actual = stream_to_ids(dbg_build_tree::<T>().traverse_depth_first()).await;
    assert_eq!(actual, treecase.dfs);
}

fn dbg_build_tree<T>() -> Tree
where
    T: TreeBuilder,
{
    dbg!(T::build_tree())
}

async fn stream_to_ids<S>(stream: S) -> Vec<u64>
where
    S: tokio_stream::Stream<Item = Result<Tree, std::convert::Infallible>>,
{
    use tokio_stream::StreamExt;

    stream
        .map(|treeres| treeres.unwrap())
        .map(|tree| tree.id)
        .collect()
        .await
}
