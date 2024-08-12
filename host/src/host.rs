use crate::{
    store::{HostLayer, HostLink},
    State,
};
use pangalactic_store::Store;
use wasmtime::{Engine, Linker, Module};

pub(crate) struct Host<S>
where
    S: Store,
{
    engine: Engine,
    linker: Linker<State<S>>,
}

impl<S> Host<S>
where
    S: Store,
{
    pub fn new() -> anyhow::Result<Self> {
        let mut config = wasmtime::Config::new();

        config
            // We rely on an async API:
            .async_support(true)
            // Pangalactic WASM is non-threaded for determinism:
            .wasm_threads(false)
            // NAN canonicalization is required for determinism:
            .cranelift_nan_canonicalization(true);

        let engine = Engine::new(&config)?;
        let linker = crate::hostapi::instantiate_linker(&engine)?;

        Ok(Host { engine, linker })
    }

    pub async fn execute(
        &mut self,
        store: HostLayer<S>,
        plan: &HostLink<S::CID>,
    ) -> anyhow::Result<(HostLayer<S>, HostLink<S::CID>)> {
        use crate::DeriveFunc;

        let mut state = State::new(store);
        let execmod = load_exec_mod(&mut state, &self.engine, plan).await?;
        let derivefunc = DeriveFunc::new(&self.engine, &self.linker, state, &execmod).await?;

        derivefunc.call_async(plan).await
    }
}

async fn load_exec_mod<S>(
    state: &mut State<S>,
    engine: &Engine,
    plan: &HostLink<S::CID>,
) -> anyhow::Result<Module>
where
    S: Store,
{
    use pangalactic_schemata::Plan;

    let store = state.store_mut();
    let plan: Plan<_> = store.load(plan).await?;
    let execbytes: Vec<u8> = store.load(&plan.exec).await?;
    let execmod = Module::new(engine, execbytes)?;
    Ok(execmod)
}
