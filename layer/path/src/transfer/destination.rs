use std::fmt::Debug;
use std::path::PathBuf;

use pangalactic_iowrappers::Writable;

use crate::{AnyDestination, StoreDestination, StorePath};

pub trait Destination: std::fmt::Debug {
    type CID;
}

impl<C> Destination for AnyDestination<C>
where
    C: serde::Serialize,
{
    type CID = Option<StorePath<C>>;
}

impl<C> Destination for StoreDestination<C>
where
    C: serde::Serialize,
{
    type CID = StorePath<C>;
}

impl<W> Destination for Writable<W>
where
    W: Debug,
{
    type CID = ();
}

impl Destination for PathBuf {
    type CID = ();
}
