use pangalactic_dir::HostDirectory;
use pangalactic_layer_cidmeta::{CidMeta, Writer};
use pangalactic_link::Link;
use pangalactic_store::Store;

pub type LinkFor<S> = Link<CidMeta<S>>;
pub type DirectoryFor<S> = HostDirectory<CidMeta<S>>;
pub type WriterFor<S> = Writer<<S as Store>::Writer>;
