use dagwasm_blobstore::BlobStore;
use dagwasm_dagio::{Dagio, LinkFor};
use wasmtime::{Engine, Store};

#[allow(dead_code)]
pub struct Host<BS> {
    engine: Engine,
    store: Store<Dagio<BS>>,
}

impl<BS> Host<BS>
where
    BS: BlobStore,
{
    pub fn new(blobstore: BS) -> anyhow::Result<Self> {
        let mut config = wasmtime::Config::new();

        config
            // We rely on an async API:
            .async_support(true)
            // DAGWASM is non-threaded for determinism:
            .wasm_threads(false)
            // NAN canonicalization is required for determinism:
            .cranelift_nan_canonicalization(true);

        let engine = Engine::new(&config)?;
        let store = Store::new(&engine, Dagio::from(blobstore));

        Ok(Host { engine, store })
    }

    pub fn execute(&self, _modlink: LinkFor<BS>) -> anyhow::Result<()> {
        todo!()
    }
}
