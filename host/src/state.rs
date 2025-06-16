use crate::{
    store::{HostDir, HostLayer, HostLink, HostWriter},
    ByteReader, DirectoryReader,
};
use pangalactic_store::Store;
use pangalactic_table::Table;

pub struct State<S>
where
    S: Store,
{
    store: HostLayer<S>,
    links: Table<HostLink<S::CID>>,
    byte_readers: Table<ByteReader<S>>,
    dir_readers: Table<DirectoryReader<S>>,
    byte_writers: Table<HostWriter<S>>,
    dir_writers: Table<HostDir<S::CID>>,
}

impl<S> State<S>
where
    S: Store,
{
    pub(crate) fn new(store: HostLayer<S>) -> Self {
        State {
            store,
            links: Table::default(),
            byte_readers: Table::default(),
            dir_readers: Table::default(),
            byte_writers: Table::default(),
            dir_writers: Table::default(),
        }
    }

    pub(crate) fn unwrap_store(self) -> HostLayer<S> {
        self.store
    }

    pub(crate) fn store_mut(&mut self) -> &mut HostLayer<S> {
        &mut self.store
    }

    pub(crate) fn links(&self) -> &Table<HostLink<S::CID>> {
        &self.links
    }

    pub(crate) fn links_mut(&mut self) -> &mut Table<HostLink<S::CID>> {
        &mut self.links
    }

    pub(crate) fn byte_readers_mut(&mut self) -> &mut Table<ByteReader<S>> {
        &mut self.byte_readers
    }

    pub(crate) fn directory_readers(&self) -> &Table<DirectoryReader<S>> {
        &self.dir_readers
    }

    pub(crate) fn directory_readers_mut(&mut self) -> &mut Table<DirectoryReader<S>> {
        &mut self.dir_readers
    }

    pub(crate) fn byte_writers_mut(&mut self) -> &mut Table<HostWriter<S>> {
        &mut self.byte_writers
    }

    pub(crate) fn directory_writers_mut(&mut self) -> &mut Table<HostDir<S::CID>> {
        &mut self.dir_writers
    }
}
