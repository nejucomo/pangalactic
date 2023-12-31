use pangalactic_hostdir::HostDirectory;
use pangalactic_layer_cidmeta::CidMeta;
use pangalactic_link::Link;

pub type DagioLink<S> = Link<CidMeta<S>>;
pub type DagioHostDirectory<S> = HostDirectory<CidMeta<S>>;
