mod inner;
mod layer;

pub use self::layer::HostLayer;

pub type HostAnyDestination<S> = pangalactic_layer_path::AnyDestination<inner::Cid<S>>;
pub type HostAnySource<S> = pangalactic_layer_path::AnySource<inner::Cid<S>>;
pub type HostStorePath<S> = pangalactic_layer_path::StorePath<inner::Cid<S>>;
pub type HostStoreDirectory<S> = pangalactic_layer_dir::StoreDirectory<inner::Cid<S>>;
