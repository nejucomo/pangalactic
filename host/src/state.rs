use dagwasm_blobstore::BlobStore;
use dagwasm_dagio::{Dagio, LinkFor};
use dagwasm_handle::Handle;
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
    pub(crate) fn new(blobstore: B, derivation: &LinkFor<B>) -> (Self, Handle<LinkFor<B>>) {
        let mut links = Table::default();
        let handle = links.insert(derivation.clone());

        let me = State {
            dagio: Dagio::from(blobstore),
            links,
        };

        (me, handle)
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
}
