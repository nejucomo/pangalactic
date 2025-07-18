use crate::{
    store::{HostLayer, HostLink},
    State,
};
use pangalactic_handle::Handle;
use pangalactic_store::Store;
use wasmtime::{Engine, Linker, Module, TypedFunc};

type RawLinkHandle = u64;

pub(crate) struct DeriveFunc<S>
where
    S: Store,
{
    store: wasmtime::Store<State<S>>,
    tfunc: TypedFunc<(RawLinkHandle,), (RawLinkHandle,)>,
}

impl<S> DeriveFunc<S>
where
    S: Store,
{
    pub(crate) async fn new(
        engine: &Engine,
        linker: &Linker<State<S>>,
        state: State<S>,
        execmod: &Module,
    ) -> anyhow::Result<Self> {
        let mut store = wasmtime::Store::new(engine, state);
        let instance = linker.instantiate_async(&mut store, execmod).await?;
        let tfunc = instance.get_typed_func::<(RawLinkHandle,), (RawLinkHandle,), _>(
            &mut store,
            "prim_derive_impl",
        )?;

        Ok(DeriveFunc { store, tfunc })
    }

    pub(crate) async fn call_async(
        mut self,
        plan: &HostLink<S::CID>,
    ) -> anyhow::Result<(HostLayer<S>, HostLink<S::CID>)> {
        use pangalactic_schemata::Attestation;

        let derive_handle = self.store.data_mut().links_mut().insert(plan.clone());
        let derive_handle_raw = unsafe { derive_handle.peek() };

        let (raw_output,): (RawLinkHandle,) = self
            .tfunc
            .call_async(&mut self.store, (derive_handle_raw,))
            .await?;
        let output_handle = unsafe { Handle::wrap(raw_output) };
        let output_link = self.store.data().links().lookup(output_handle).cloned()?;

        let mut store = self.store.into_data().unwrap_store();
        let attestation_link = store
            .commit(Attestation {
                plan: plan.clone(),
                output: output_link,
            })
            .await?;

        Ok((store, attestation_link))
    }
}
