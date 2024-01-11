//! Asynchronous Directed Acyclic Graph traversal streams

#![feature(try_trait_v2, box_into_inner)]
#![deny(unsafe_code, missing_docs)]

mod bfs;
mod traversable;

pub use self::bfs::TraverseBreadthFirst;
pub use self::traversable::TraversableDag;

#[cfg(test)]
mod tests;
