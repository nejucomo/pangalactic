use pangalactic_node::{Dir, Link};
use pangalactic_store::KeyOf;

pub type DirFor<S> = Dir<KeyOf<S>>;
pub type LinkFor<S> = Link<KeyOf<S>>;
