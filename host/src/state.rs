use dagwasm_blobstore::BlobStore;
use dagwasm_dagio::{Dagio, LinkFor};
use dagwasm_table::Table;

#[derive(Debug)]
pub struct State<B>
where
    B: BlobStore,
{
    dagio: Dagio<B>,
    links: Table<LinkFor<B>>,
}

impl<B> State<B>
where
    B: BlobStore,
{
    pub(crate) fn new(dagio: Dagio<B>) -> Self {
        State {
            dagio,
            links: Table::default(),
        }
    }

    pub(crate) fn unwrap_dagio(self) -> Dagio<B> {
        self.dagio
    }

    #[allow(dead_code)]
    pub(crate) fn dagio(&self) -> &Dagio<B> {
        &self.dagio
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
}
