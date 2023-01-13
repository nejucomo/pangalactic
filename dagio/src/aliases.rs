use dagwasm_dir::Directory;
use dagwasm_link::Link;
use dagwasm_store::Store;

pub type LinkFor<S> = Link<<S as Store>::CID>;
pub type DirectoryFor<S> = Directory<<S as Store>::CID>;
