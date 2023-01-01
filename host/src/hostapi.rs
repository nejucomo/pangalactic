use crate::{HostToWasm, State, WasmToHost};
use dagwasm_blobstore::BlobStore;
use dagwasm_dagio::LinkFor;
use wasmtime::{Caller, Engine, Linker, Trap};

pub(crate) fn instantiate_linker<B>(engine: &Engine) -> anyhow::Result<Linker<State<B>>>
where
    B: BlobStore,
{
    const HOSTMOD: &str = env!("CARGO_PKG_NAME");

    let mut linker = Linker::new(engine);

    macro_rules! link_host_fn {
        ( method $wrapmethod:ident, $name:ident, $( $arg:ident ),* ) => {
            linker . $wrapmethod(
                dbg!(HOSTMOD),
                dbg!(stringify!($name)),
                |caller: Caller<'_, State<B>>, $( $arg : u64 ),* | Box::new($name(caller, $( $arg ),* )),
            )
        };

        ( $name:ident, $a0:ident ) => {
            link_host_fn!(method func_wrap1_async, $name, $a0)
        }
    }

    link_host_fn!(link_get_kind, link)?;
    link_host_fn!(link_open_directory_reader, link)?;

    Ok(linker)
}

async fn link_get_kind<B>(caller: Caller<'_, State<B>>, rh_link: u64) -> Result<u64, Trap>
where
    B: BlobStore,
{
    use dagwasm_handle::Handle;

    let h_link: Handle<LinkFor<B>> = rh_link.into_host();
    let link = caller.data().links().lookup(h_link)?;
    Ok(link.kind().into_wasm())
}

async fn link_open_directory_reader<B>(
    mut caller: Caller<'_, State<B>>,
    rh_link: u64,
) -> Result<u64, Trap>
where
    B: BlobStore,
{
    use crate::DirectoryReader;
    use dagwasm_handle::Handle;

    let h_link: Handle<LinkFor<B>> = rh_link.into_host();
    let link = caller.data().links().lookup(h_link)?.clone();
    let dr: DirectoryReader<B> = caller.data_mut().dagio_mut().read(&link).await?;
    let h_dr = caller.data_mut().directory_readers_mut().insert(dr);
    Ok(h_dr.into_wasm())
}
