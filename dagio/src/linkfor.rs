use dagwasm_dir::Link;
use dagwasm_store::Store;

pub type LinkFor<S> = Link<<S as Store>::CID>;
