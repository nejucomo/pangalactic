use test_case::test_case;

mod tree;

use self::tree::{NodeMaker as N, Tree, TreeBuilder};
use crate::TraversableDag;

#[test_case(N => Vec::<u64>::default() ; "singleton")]
#[test_case((N, N) => vec![1, 2] ; "v-tree")]
#[test_case(((N, N), N) => vec![1, 4] ; "v-v-node")]
#[test_case((N, (N, N)) => vec![1, 2] ; "v-node-v")]
#[tokio::test]
async fn children<T>(mktree: T) -> Vec<u64>
where
    T: TreeBuilder,
{
    stream_to_ids(dbg_build_tree(mktree).children().await.unwrap()).await
}

#[test_case(N => vec![0] ; "singleton")]
#[test_case((N, N) => vec![0, 1, 2] ; "v-tree")]
#[test_case(((N, N), N) => vec![0, 1, 4, 2, 3] ; "v-v-node")]
#[test_case((N, (N, N)) => vec![0, 1, 2, 3, 4] ; "v-node-v")]
#[tokio::test]
async fn bfs<T>(mktree: T) -> Vec<u64>
where
    T: TreeBuilder,
{
    stream_to_ids(dbg_build_tree(mktree).traverse_breadth_first()).await
}

#[test_case(N => vec![0] ; "singleton")]
#[test_case((N, N) => vec![1, 2, 0] ; "v-tree")]
#[test_case(((N, N), N) => vec![2, 3, 1, 4, 0] ; "v-v-node")]
#[test_case((N, (N, N)) => vec![1, 3, 4, 2, 0] ; "v-node-v")]
#[tokio::test]
async fn dfs<T>(mktree: T) -> Vec<u64>
where
    T: TreeBuilder,
{
    stream_to_ids(dbg_build_tree(mktree).traverse_depth_first()).await
}

fn dbg_build_tree<T>(mktree: T) -> Tree
where
    T: TreeBuilder,
{
    dbg!(dbg!(mktree).build_tree())
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
