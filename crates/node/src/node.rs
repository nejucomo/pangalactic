use pangalactic_std_store::StdStore;

use crate::NodeOptions;

#[derive(Debug)]
pub struct Node {
    #[allow(dead_code)]
    store: StdStore,
}

impl From<NodeOptions> for Node {
    fn from(opts: NodeOptions) -> Self {
        Node {
            store: StdStore::from(opts.dirdb),
        }
    }
}
