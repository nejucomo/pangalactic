use dagwasm_dir::HostDirectory;
use dagwasm_link::Link;
use dagwasm_store::Store;

pub type LinkFor<S> = Link<<S as Store>::CID>;
pub type DirectoryFor<S> = HostDirectory<<S as Store>::CID>;
