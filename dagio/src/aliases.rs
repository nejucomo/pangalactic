use dagwasm_dir::HostDirectory;
use dagwasm_layer_cidmeta::{CidMeta, Writer};
use dagwasm_link::Link;
use dagwasm_store::Store;

pub type LinkFor<S> = Link<CidMeta<S>>;
pub type DirectoryFor<S> = HostDirectory<CidMeta<S>>;
pub type WriterFor<S> = Writer<<S as Store>::Writer>;
