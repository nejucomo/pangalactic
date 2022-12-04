use dagwasm_blobstore::BlobStore;
use dagwasm_dagio::LinkFor;
use dagwasm_handle::Handle;
use wasmtime::{Engine, Store};

#[allow(dead_code)]
pub struct Host {
    engine: Engine,
}

impl Host {
    pub fn new() -> anyhow::Result<Self> {
        let mut config = wasmtime::Config::new();

        config
            // We rely on an async API:
            .async_support(true)
            // DAGWASM is non-threaded for determinism:
            .wasm_threads(false)
            // NAN canonicalization is required for determinism:
            .cranelift_nan_canonicalization(true);

        let engine = Engine::new(&config)?;
        Ok(Host { engine })
    }

    pub async fn execute<BS>(
        &mut self,
        blobstore: BS,
        derivation: &LinkFor<BS>,
    ) -> anyhow::Result<LinkFor<BS>>
    where
        BS: BlobStore,
    {
        use crate::State;
        use dagwasm_dagify::FromDag;
        use dagwasm_derivation::Derivation;
        use wasmtime::{Instance, Module};

        let (state, handle) = State::new(blobstore, derivation);
        let mut store = Store::new(&self.engine, state);
        let state = store.data_mut();
        let dagio = state.dagio_mut();
        let deriv = Derivation::from_dag(dagio, derivation).await?;
        let execbytes = dagio.read_file(&deriv.exec).await?;
        let execmod = Module::new(&self.engine, execbytes)?;
        let instance = Instance::new(&mut store, &execmod, &[])?;

        type RawHandle = u64;
        let derivefunc =
            instance.get_typed_func::<(RawHandle,), (RawHandle,), _>(&mut store, "derive")?;

        let raw_input = unsafe { handle.peek() };
        let (raw_output,): (RawHandle,) = derivefunc.call_async(&mut store, (raw_input,)).await?;
        let output_handle = unsafe { Handle::wrap(raw_output) };
        let output_link = store.data().links().lookup(output_handle).cloned()?;
        Ok(output_link)
    }
}
