use dagwasm_blobstore::BlobStore;
use dagwasm_dir::Link;

pub type LinkFor<B> = Link<<B as BlobStore>::Key>;
