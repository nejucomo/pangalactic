use pangalactic_dagio::{Dagio, LinkFor};
use pangalactic_path::AnyPath;
use pangalactic_store::Store;
use pangalactic_store_dirdb::DirDbStore;

pub type CliDagio = Dagio<DirDbStore>;
pub type CliLink = LinkFor<DirDbStore>;
pub type CliPath = AnyPath<<DirDbStore as Store>::Cid>;
