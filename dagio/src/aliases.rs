use pangalactic_hostdir::HostDirectory;
use pangalactic_layer_cidmeta::{CidMeta, Writer};
use pangalactic_link::Link;
use pangalactic_store::Store;

pub type DagioLink<S> = Link<CidMeta<S>>;
pub type DagioHostDirectory<S> = HostDirectory<CidMeta<S>>;
pub type DagioWriter<S> = Writer<<S as Store>::Writer>;
