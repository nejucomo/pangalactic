use pangalactic_cryptostore::CryptoStore;
use pangalactic_dirstore::DirStore;
use pangalactic_store::Store;

pub struct PgStore(CryptoStore<DirStore>);

impl PgStore {
    pub fn open() -> IOResult<PgStore> {}
}
