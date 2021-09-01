use pangalactic_cryptostore::ReadCap;
use pangalactic_dirstore as dirstore;
use pangalactic_node as node;

pub type Dir = node::Dir<Key>;
pub type Entry = node::Entry<Key>;
pub type Kind = node::Kind;
pub type Link = node::Link<Key>;

type Key = ReadCap<dirstore::Key>;
