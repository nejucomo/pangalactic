use dagwasm_dir::Link;
use dagwasm_store::Store;

pub type LinkFor<B> = Link<<B as Store>::CID>;
