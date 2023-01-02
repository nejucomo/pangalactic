use crate::State;
use dagwasm_blobstore::BlobStore;
use dagwasm_dagio::{Dagio, LinkFor};
use dagwasm_handle::Handle;
use std::ops::Deref;
use wasmtime::{Engine, Linker, Module, Store, TypedFunc};

type RawLinkHandle = u64;

pub(crate) struct DeriveFunc<B>
where
    B: BlobStore,
    <B as BlobStore>::Writer: Deref,
    <<B as BlobStore>::Writer as Deref>::Target: Unpin,
{
    store: Store<State<B>>,
    tfunc: TypedFunc<(RawLinkHandle,), (RawLinkHandle,)>,
}

impl<B> DeriveFunc<B>
where
    B: BlobStore,
    <B as BlobStore>::Writer: Deref,
    <<B as BlobStore>::Writer as Deref>::Target: Unpin,
{
    pub(crate) async fn new(
        engine: &Engine,
        linker: &Linker<State<B>>,
        state: State<B>,
        execmod: &Module,
    ) -> anyhow::Result<Self> {
        let mut store = Store::new(engine, state);
        let instance = linker.instantiate_async(&mut store, execmod).await?;
        let tfunc = instance
            .get_typed_func::<(RawLinkHandle,), (RawLinkHandle,), _>(&mut store, "derive")?;

        Ok(DeriveFunc { store, tfunc })
    }

    pub(crate) async fn call_async(
        mut self,
        derivation: &LinkFor<B>,
    ) -> anyhow::Result<(Dagio<B>, LinkFor<B>)> {
        use dagwasm_schemata::Attestation;

        let derive_handle = self.store.data_mut().links_mut().insert(derivation.clone());
        let derive_handle_raw = unsafe { derive_handle.peek() };

        let (raw_output,): (RawLinkHandle,) = self
            .tfunc
            .call_async(&mut self.store, (derive_handle_raw,))
            .await?;
        let output_handle = unsafe { Handle::wrap(raw_output) };
        let output_link = self.store.data().links().lookup(output_handle).cloned()?;

        let mut dagio = self.store.into_data().unwrap_dagio();
        let attestation_link = dagio
            .commit(Attestation {
                derivation: derivation.clone(),
                output: output_link,
            })
            .await?;

        Ok((dagio, attestation_link))
    }
}
