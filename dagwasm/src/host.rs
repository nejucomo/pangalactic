use crate::dag;
use wasmtime::{Engine, Store};

#[allow(dead_code)]
pub struct Host<DS> {
    engine: Engine,
    store: Store<DS>,
}

impl<DS> Host<DS>
where
    DS: dag::Store,
{
    pub fn new(dagstore: DS) -> anyhow::Result<Self> {
        let mut config = wasmtime::Config::new();

        config
            // We rely on an async API:
            .async_support(true)
            // DAGWASM is non-threaded for determinism:
            .wasm_threads(false)
            // NAN canonicalization is required for determinism:
            .cranelift_nan_canonicalization(true);

        let engine = Engine::new(&config)?;
        let store = Store::new(&engine, dagstore);

        Ok(Host { engine, store })
    }

    pub fn execute(&self, _modlink: <DS as dag::Store>::Link) -> anyhow::Result<()> {
        todo!()
    }
}
