use crate::State;
use dagwasm_blobstore::BlobStore;
use dagwasm_dagio::LinkFor;
use dagwasm_handle::Handle;
use wasmtime::{Engine, Module, Store, TypedFunc};

type RawLinkHandle = u64;

pub(crate) struct DeriveFunc<B>
where
    B: BlobStore,
{
    store: Store<State<B>>,
    tfunc: TypedFunc<(RawLinkHandle,), (RawLinkHandle,)>,
}

impl<B> DeriveFunc<B>
where
    B: BlobStore,
{
    pub(crate) fn new(engine: &Engine, state: State<B>, execmod: &Module) -> anyhow::Result<Self> {
        use wasmtime::Instance;

        let mut store = Store::new(engine, state);
        let instance = Instance::new(&mut store, execmod, &[])?;
        let tfunc = instance
            .get_typed_func::<(RawLinkHandle,), (RawLinkHandle,), _>(&mut store, "derive")?;

        Ok(DeriveFunc { store, tfunc })
    }

    pub(crate) async fn call_async(
        &mut self,
        derivation: &LinkFor<B>,
    ) -> anyhow::Result<LinkFor<B>> {
        let derive_handle = self.store.data_mut().links_mut().insert(derivation.clone());
        let derive_handle_raw = unsafe { derive_handle.peek() };

        let (raw_output,): (RawLinkHandle,) = self
            .tfunc
            .call_async(&mut self.store, (derive_handle_raw,))
            .await?;
        let output_handle = unsafe { Handle::wrap(raw_output) };
        let output_link = self.store.data().links().lookup(output_handle).cloned()?;
        Ok(output_link)
    }
}
