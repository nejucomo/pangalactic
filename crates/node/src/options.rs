use clap::Parser;
use derive_more::{From, Into};
use pangalactic_store_dirdb::DirDbStore;

#[derive(Clone, Debug, Parser, From, Into)]
pub struct NodeOptions {
    /// The path to the dirdb store directory
    ///
    // TODO API gap: the `dirdb` arg type should be `StdStore`
    #[clap(short, long, default_value_t)]
    pub dirdb: DirDbStore,
}
