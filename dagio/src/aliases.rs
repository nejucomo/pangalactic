use pangalactic_hostdir::HostDirectory;
use pangalactic_layer_cidmeta::CidMetaLayer;
use pangalactic_link::Link;
use pangalactic_storepath::StorePath;

pub type DagioLink<S> = Link<CidMetaLayer<S>>;
pub type DagioHostDirectory<S> = HostDirectory<CidMetaLayer<S>>;
pub type DagioStorePath<S> = StorePath<CidMetaLayer<S>>;
