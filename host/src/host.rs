use crate::State;
use dagwasm_blobstore::BlobStore;
use dagwasm_dagio::LinkFor;
use wasmtime::{Engine, Module};

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

    pub async fn execute<B>(
        &mut self,
        blobstore: B,
        derivation: &LinkFor<B>,
    ) -> anyhow::Result<LinkFor<B>>
    where
        B: BlobStore,
    {
        use crate::DeriveFunc;

        let mut state = State::new(blobstore);
        let execmod = load_exec_mod(&mut state, &self.engine, derivation).await?;
        let mut derivefunc = DeriveFunc::new(&self.engine, state, &execmod)?;

        derivefunc.call_async(derivation).await
    }
}

async fn load_exec_mod<B>(
    state: &mut State<B>,
    engine: &Engine,
    derivation: &LinkFor<B>,
) -> anyhow::Result<Module>
where
    B: BlobStore,
{
    use dagwasm_dagify::FromDag;
    use dagwasm_derivation::Derivation;

    let dagio = state.dagio_mut();
    let deriv = Derivation::from_dag(dagio, derivation).await?;
    let execbytes = dagio.read_file(&deriv.exec).await?;
    let execmod = Module::new(engine, execbytes)?;
    Ok(execmod)
}
