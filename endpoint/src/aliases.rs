use pangalactic_dag_transfer::{IntoSource, Source};
use pangalactic_layer_dir::{DirectoryIntoIter, LinkDirectoryLayer};
use pangalactic_link::Link;
use pangalactic_linkpath::{LinkDestination, LinkPath};
use pangalactic_store::Store;
use tokio::fs::{File, ReadDir};

use crate::{endpoint::Endpoint, HostOrStore, HostPath, Stdio};

pub type OriginEndpoint<C> = Endpoint<Stdio, HostPath, LinkPath<C>>;
pub type DestinationEndpoint<C> = Endpoint<Stdio, HostPath, LinkDestination<C>>;
pub type Receipt<C> = Endpoint<Stdio, HostPath, LinkPath<C>>;

pub type OriginEndpointSource<S> = Source<OriginEndpointLeaf<S>, OriginEndpointBranch<S>>;

pub type OriginEndpointLeaf<S> =
    Endpoint<Stdio, File, <LinkPath<<S as Store>::CID> as IntoSource<S>>::Leaf>;

pub type OriginEndpointBranch<S> = HostOrStore<
    <HostPath as IntoSource<S>>::Branch,
    <LinkPath<<S as Store>::CID> as IntoSource<S>>::Branch,
>;

pub type HostPathSource = Source<File, ReadDir>;
pub type LinkPathSource<S> = Source<LinkPathLeaf<S>, LinkPathBranch<S>>;
pub type LinkPathLeaf<S> = <LinkDirectoryLayer<S> as Store>::Reader;
pub type LinkPathBranch<S> = DirectoryIntoIter<Link<<S as Store>::CID>>;
