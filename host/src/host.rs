use crate::State;
use dagwasm_dagio::{Dagio, LinkFor};
use dagwasm_store::Store;
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
        dagio: Dagio<S>,
        plan: &LinkFor<S>,
    ) -> anyhow::Result<(Dagio<S>, LinkFor<S>)> {
        use crate::DeriveFunc;

        let mut state = State::new(dagio);
        let execmod = load_exec_mod(&mut state, &self.engine, plan).await?;
        let derivefunc = DeriveFunc::new(&self.engine, &self.linker, state, &execmod).await?;

        derivefunc.call_async(plan).await
    }
}

async fn load_exec_mod<S>(
    state: &mut State<S>,
    engine: &Engine,
    plan: &LinkFor<S>,
) -> anyhow::Result<Module>
where
    S: Store,
{
    use dagwasm_dagio::FromDag;
    use dagwasm_schemata::Plan;

    let dagio = state.dagio_mut();
    let plan = Plan::from_dag(dagio, plan).await?;
    let execbytes = dagio.read_file(&plan.exec).await?;
    let execmod = Module::new(engine, execbytes)?;
    Ok(execmod)
}
