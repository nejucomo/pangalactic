mod hosttree;
mod node;

use pangalactic_layer_cidmeta::CidMetaLayer;
use pangalactic_store::Store;
use pangalactic_storepath::{StoreDestination, StorePath};

pub type HostTreePath<S> = StorePath<<CidMetaLayer<S> as Store>::Cid>;
pub type HostTreeDestination<S> = StoreDestination<<CidMetaLayer<S> as Store>::Cid>;

pub use self::hosttree::HostTree;
pub(crate) use self::node::TreeNode;
