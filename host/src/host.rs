use crate::State;
use dagwasm_blobstore::BlobStore;
use dagwasm_dagio::{Dagio, LinkFor};
use std::ops::Deref;
use wasmtime::{Engine, Linker, Module};

pub async fn derive<B>(
    dagio: Dagio<B>,
    derivation: &LinkFor<B>,
) -> anyhow::Result<(Dagio<B>, LinkFor<B>)>
where
    B: BlobStore,
    <B as BlobStore>::Writer: Deref,
    <<B as BlobStore>::Writer as Deref>::Target: Unpin,
{
    let mut host = Host::new()?;
    host.execute(dagio, derivation).await
}

struct Host<B>
where
    B: BlobStore,
    <B as BlobStore>::Writer: Deref,
    <<B as BlobStore>::Writer as Deref>::Target: Unpin,
{
    engine: Engine,
    linker: Linker<State<B>>,
}

impl<B> Host<B>
where
    B: BlobStore,
    <B as BlobStore>::Writer: Deref,
    <<B as BlobStore>::Writer as Deref>::Target: Unpin,
{
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
        let linker = crate::hostapi::instantiate_linker(&engine)?;

        Ok(Host { engine, linker })
    }

    pub async fn execute(
        &mut self,
        dagio: Dagio<B>,
        derivation: &LinkFor<B>,
    ) -> anyhow::Result<(Dagio<B>, LinkFor<B>)> {
        use crate::DeriveFunc;

        let mut state = State::new(dagio);
        let execmod = load_exec_mod(&mut state, &self.engine, derivation).await?;
        let derivefunc = DeriveFunc::new(&self.engine, &self.linker, state, &execmod).await?;

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
    use dagwasm_dagio::FromDag;
    use dagwasm_schemata::Derivation;

    let dagio = state.dagio_mut();
    let deriv = Derivation::from_dag(dagio, derivation).await?;
    let execbytes = dagio.read_file(&deriv.exec).await?;
    let execmod = Module::new(engine, execbytes)?;
    Ok(execmod)
}
