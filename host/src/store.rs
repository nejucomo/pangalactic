use pangalactic_layer_cidmeta::{CidMeta, CidMetaLayer};
use pangalactic_layer_dir::{LinkDirectory, LinkDirectoryLayer};
use pangalactic_link::Link;
use pangalactic_store::Store;

pub type HostLayer<S> = LinkDirectoryLayer<CidMetaLayer<S>>;
pub type HostLink<C> = Link<CidMeta<C>>;
pub type HostDir<C> = LinkDirectory<CidMeta<C>>;
pub type HostWriter<S> = <HostLayer<S> as Store>::Writer;
pub type HostReader<S> = <HostLayer<S> as Store>::Reader;
