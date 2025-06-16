mod stdlayer;

use pangalactic_endpoint::{DestinationEndpoint, OriginEndpoint};
use pangalactic_hash::Hash;
use pangalactic_layer_cidmeta::CidMeta;
use pangalactic_link::Link;

pub use self::stdlayer::{StdLayer, StdMemStore, StdStore};

pub type StdLink = Link<StdCid>;
pub type StdCid = CidMeta<Hash>;

pub type StdOrigin = OriginEndpoint<StdCid>;
pub type StdDestination = DestinationEndpoint<StdCid>;
