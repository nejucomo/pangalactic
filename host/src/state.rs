use crate::{ByteReader, DirectoryReader};
use dagwasm_dagio::{Dagio, LinkFor};
use dagwasm_store::Store;
use dagwasm_table::Table;

pub struct State<B>
where
    B: Store,
{
    dagio: Dagio<B>,
    links: Table<LinkFor<B>>,
    byte_readers: Table<ByteReader<B>>,
    dir_readers: Table<DirectoryReader<B>>,
}

impl<B> State<B>
where
    B: Store,
{
    pub(crate) fn new(dagio: Dagio<B>) -> Self {
        State {
            dagio,
            links: Table::default(),
            byte_readers: Table::default(),
            dir_readers: Table::default(),
        }
    }

    pub(crate) fn unwrap_dagio(self) -> Dagio<B> {
        self.dagio
    }

    pub(crate) fn dagio_mut(&mut self) -> &mut Dagio<B> {
        &mut self.dagio
    }

    pub(crate) fn links(&self) -> &Table<LinkFor<B>> {
        &self.links
    }

    pub(crate) fn links_mut(&mut self) -> &mut Table<LinkFor<B>> {
        &mut self.links
    }

    pub(crate) fn byte_readers_mut(&mut self) -> &mut Table<ByteReader<B>> {
        &mut self.byte_readers
    }

    pub(crate) fn directory_readers(&self) -> &Table<DirectoryReader<B>> {
        &self.dir_readers
    }

    pub(crate) fn directory_readers_mut(&mut self) -> &mut Table<DirectoryReader<B>> {
        &mut self.dir_readers
    }
}
