use crate::State;
use dagwasm_blobstore::BlobStore;
use wasmtime::{Engine, Linker};

pub(crate) fn instantiate_linker<B>(engine: &Engine) -> anyhow::Result<Linker<State<B>>>
where
    B: BlobStore,
{
    use wasmtime::Caller;

    const HOSTMOD: &str = env!("CARGO_PKG_NAME");

    let mut linker = Linker::new(engine);
    linker.func_wrap1_async(
        HOSTMOD,
        "ident",
        |_caller: Caller<'_, State<B>>, param: u64| Box::new(async move { param }),
    )?;

    Ok(linker)
}
