use pangalactic_dagfs::Dagfs;
use pangalactic_dagio::HostDirectoryFor;
use pangalactic_layer_cidmeta::CidMeta;
use pangalactic_link::Link;
use pangalactic_store_dirdb::DirDbStore;
use pangalactic_storepath::{StoreDestination, StorePath};

pub type CliDagfs = Dagfs<DirDbStore>;
pub type CliDirectory = HostDirectoryFor<DirDbStore>;
pub type CliLink = Link<CidMeta<DirDbStore>>;
pub type CliStorePath = StorePath<CidMeta<DirDbStore>>;
pub type CliStoreDestination = StoreDestination<CidMeta<DirDbStore>>;
