use pangalactic_dagio::{Dagio, LinkFor};
use pangalactic_store_dirdb::DirDbStore;

pub type CliDagio = Dagio<DirDbStore>;
pub type CliLink = LinkFor<DirDbStore>;
