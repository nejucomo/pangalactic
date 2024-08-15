mod inner;
mod layer;

pub use self::layer::HostLayer;

pub type HostAnyDestination<S> = pangalactic_layer_path::AnyDestination<inner::Cid<S>>;
pub type HostAnySource<S> = pangalactic_layer_path::AnySource<inner::Cid<S>>;
/// BUG: See [HostLayer::resolve_path] BUG comment.
pub type HostLink<S> = pangalactic_link::Link<inner::Cid<S>>;
pub type HostLinkDirectory<S> = pangalactic_layer_dir::LinkDirectory<inner::Cid<S>>;
pub type HostStorePath<S> = pangalactic_layer_path::StorePath<inner::Cid<S>>;
