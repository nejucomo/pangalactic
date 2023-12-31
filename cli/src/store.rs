use pangalactic_dagio::{Dagio, DagioLink, DagioReader};
use pangalactic_store_dirdb::DirDbStore;

pub type CliDagio = Dagio<DirDbStore>;
pub type CliLink = DagioLink<DirDbStore>;
pub type CliReader = DagioReader<DirDbStore>;
