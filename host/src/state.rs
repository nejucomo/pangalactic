use crate::{ByteReader, DirectoryReader};
use pangalactic_dagio::{Dagio, DagioLink, HostDirectoryFor, WriterFor};
use pangalactic_store::Store;
use pangalactic_table::Table;

pub struct State<S>
where
    S: Store,
{
    dagio: Dagio<S>,
    links: Table<DagioLink<S>>,
    byte_readers: Table<ByteReader<S>>,
    dir_readers: Table<DirectoryReader<S>>,
    byte_writers: Table<WriterFor<S>>,
    dir_writers: Table<HostDirectoryFor<S>>,
}

impl<S> State<S>
where
    S: Store,
{
    pub(crate) fn new(dagio: Dagio<S>) -> Self {
        State {
            dagio,
            links: Table::default(),
            byte_readers: Table::default(),
            dir_readers: Table::default(),
            byte_writers: Table::default(),
            dir_writers: Table::default(),
        }
    }

    pub(crate) fn unwrap_dagio(self) -> Dagio<S> {
        self.dagio
    }

    pub(crate) fn dagio_mut(&mut self) -> &mut Dagio<S> {
        &mut self.dagio
    }

    pub(crate) fn links(&self) -> &Table<DagioLink<S>> {
        &self.links
    }

    pub(crate) fn links_mut(&mut self) -> &mut Table<DagioLink<S>> {
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

    pub(crate) fn byte_writers_mut(&mut self) -> &mut Table<WriterFor<S>> {
        &mut self.byte_writers
    }

    pub(crate) fn directory_writers_mut(&mut self) -> &mut Table<HostDirectoryFor<S>> {
        &mut self.dir_writers
    }
}
