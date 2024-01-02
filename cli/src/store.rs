use pangalactic_dagio::{Dagio, DagioLink, DagioReader};
use pangalactic_store::Store;
use pangalactic_store_dirdb::DirDbStore;
use pangalactic_storepath::{StoreDestination, StorePath};

pub type CliCid = <DirDbStore as Store>::CID;
pub type CliDagio = Dagio<DirDbStore>;
pub type CliLink = DagioLink<DirDbStore>;
pub type CliReader = DagioReader<DirDbStore>;
pub type CliStoreDestination = StoreDestination<DirDbStore>;
pub type CliStorePath = StorePath<DirDbStore>;
