use pangalactic_appdirs::AppDirs;
use pangalactic_cryptostore::CryptoStore;
use pangalactic_dirstore::DirStore;
// use pangalactic_store::Store;

#[derive(Debug)]
pub struct PgStore(CryptoStore<DirStore>);

impl PgStore {
    pub fn open() -> std::io::Result<PgStore> {
        let dirs = dbg!(AppDirs::init(crate::APP_NAME)?);
        Ok(PgStore(CryptoStore::from(DirStore::from(dirs.data))))
    }
}
