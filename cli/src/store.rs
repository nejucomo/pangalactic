use pangalactic_dagio::{Dagio, DagioLink};
use pangalactic_store_dirdb::DirDbStore;

pub type CliDagio = Dagio<DirDbStore>;
pub type CliLink = DagioLink<DirDbStore>;
