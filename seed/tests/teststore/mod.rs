use pangalactic_layer_cidmeta::{CidMeta, CidMetaLayer};
use pangalactic_layer_dir::LinkDirectoryLayer;
use pangalactic_link::Link;
use pangalactic_store::Store;
use pangalactic_store_mem::MemStore;

pub type TestStore = LinkDirectoryLayer<CidMetaLayer<MemStore>>;
pub type TestLink = Link<CidMeta<<MemStore as Store>::CID>>;
