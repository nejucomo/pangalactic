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

    pub async fn execute(&mut self, derivation: &LinkFor<BS>) -> anyhow::Result<()> {
        use dagwasm_dagify::FromDag;
        use dagwasm_derivation::Derivation;
        use wasmtime::{Instance, Module};

        let dagio = self.store.data_mut();
        let deriv = Derivation::from_dag(dagio, derivation).await?;
        let execbytes = dagio.read_file(&deriv.exec).await?;
        let execmod = Module::new(&self.engine, execbytes)?;
        let instance = Instance::new(&mut self.store, &execmod, &[])?;
        let derivefunc = instance.get_typed_func::<(), (), _>(&mut self.store, "derive")?;
        derivefunc.call(&mut self.store, ())?;
        todo!("pass derivation and handle returned output link");
    }
}
