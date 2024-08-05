use std::path::PathBuf;

use pangalactic_iowrappers::Writable;
use pangalactic_path::{AnyDestination, StoreDestination, StorePath};

pub trait Destination {
    type CID;
}

impl<C> Destination for AnyDestination<C> {
    type CID = Option<StorePath<C>>;
}

impl<C> Destination for StoreDestination<C> {
    type CID = StorePath<C>;
}

impl<W> Destination for Writable<W> {
    type CID = ();
}

impl Destination for PathBuf {
    type CID = ();
}
