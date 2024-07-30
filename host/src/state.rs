use crate::{ByteReader, DirectoryReader};
use pangalactic_dagio::{Dagio, DagioWriter};
use pangalactic_hostdir::HostDirectory;
use pangalactic_layer_cidmeta::CidMeta;
use pangalactic_link::Link;
use pangalactic_store::Store;
use pangalactic_table::Table;

pub struct State<S>
where
    S: Store,
{
    dagio: Dagio<S>,
    links: Table<Link<CidMeta<S::CID>>>,
    byte_readers: Table<ByteReader<S>>,
    dir_readers: Table<DirectoryReader<S>>,
    byte_writers: Table<DagioWriter<S>>,
    dir_writers: Table<HostDirectory<S::CID>>,
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

    pub(crate) fn links(&self) -> &Table<Link<CidMeta<S::CID>>> {
        &self.links
    }

    pub(crate) fn links_mut(&mut self) -> &mut Table<Link<CidMeta<S::CID>>> {
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

    pub(crate) fn byte_writers_mut(&mut self) -> &mut Table<DagioWriter<S>> {
        &mut self.byte_writers
    }

    pub(crate) fn directory_writers_mut(&mut self) -> &mut Table<HostDirectory<S::CID>> {
        &mut self.dir_writers
    }
}
