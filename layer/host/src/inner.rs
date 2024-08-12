use pangalactic_layer_cidmeta::{CidMeta, CidMetaLayer};
use pangalactic_layer_path::PathLayer;
use pangalactic_store::Store;

pub(crate) type Layer<S> = PathLayer<CidMetaLayer<S>>;

pub(crate) type Cid<S> = CidMeta<<S as Store>::CID>;
pub(crate) type Reader<S> = <Layer<S> as Store>::Reader;
pub(crate) type Writer<S> = <Layer<S> as Store>::Writer;
