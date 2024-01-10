use test_case::test_case;

mod tree;

use self::tree::{NodeMaker as N, TreeBuilder};
use crate::TraversableDag;
use tokio_stream::StreamExt;

#[test_case(N => Vec::<u64>::default() ; "singleton")]
#[test_case((N, N) => vec![1, 2] ; "v-tree")]
#[test_case(((N, N), N) => vec![1, 4] ; "v-v-node")]
#[test_case((N, (N, N)) => vec![1, 2] ; "v-node-v")]
#[tokio::test]
async fn children<T>(mktree: T) -> Vec<u64>
where
    T: TreeBuilder,
{
    dbg!(dbg!(mktree).build_tree())
        .children()
        .await
        .unwrap()
        .map(|treeres| treeres.unwrap())
        .map(|tree| tree.id)
        .collect()
        .await
}

/* Cases for left-biased depth-first traversal:
#[test_case(N => Vec::<u64>::default() ; "singleton")]
#[test_case((N, N) => vec![1, 2] ; "v-tree")]
#[test_case(((N, N), N) => vec![2, 3, 4] ; "v-v-node")]
#[test_case((N, (N, N)) => vec![2, 3, 4] ; "v-node-v")]
*/
