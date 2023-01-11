use crate::{HostToWasm, State, WasmToHost};
use dagwasm_dagio::LinkFor;
use dagwasm_primitives as prim;
use dagwasm_store::Store;
use wasmtime::{Caller, Trap};

pub(super) async fn get_kind<S>(
    caller: Caller<'_, State<S>>,
    rh_link: prim::HandleLink,
) -> Result<prim::LinkKind, Trap>
where
    S: Store,
{
    use dagwasm_handle::Handle;

    let h_link: Handle<LinkFor<S>> = rh_link.into_host();
    let link = caller.data().links().lookup(h_link)?;
    Ok(link.kind().into_wasm())
}

pub(super) async fn open_file_reader<S>(
    mut caller: Caller<'_, State<S>>,
    rh_link: prim::HandleLink,
) -> Result<prim::HandleByteReader, Trap>
where
    S: Store,
{
    use crate::ByteReader;
    use dagwasm_handle::Handle;

    let h_link: Handle<LinkFor<S>> = rh_link.into_host();

    let link = caller.data().links().lookup(h_link)?.clone();

    let fr = caller
        .data_mut()
        .dagio_mut()
        .open_file_reader(&link)
        .await?;

    let h_fr = caller
        .data_mut()
        .byte_readers_mut()
        .insert(ByteReader::Store(fr));

    Ok(h_fr.into_wasm())
}

pub(super) async fn open_directory_reader<S>(
    mut caller: Caller<'_, State<S>>,
    rh_link: prim::HandleLink,
) -> Result<prim::HandleDirReader, Trap>
where
    S: Store,
{
    use crate::DirectoryReader;
    use dagwasm_handle::Handle;

    let h_link: Handle<LinkFor<S>> = rh_link.into_host();
    let link = caller.data().links().lookup(h_link)?.clone();
    let dr: DirectoryReader<S> = caller.data_mut().dagio_mut().read(&link).await?;
    let h_dr = caller.data_mut().directory_readers_mut().insert(dr);
    Ok(h_dr.into_wasm())
}

pub(super) async fn close<S>(
    mut caller: Caller<'_, State<S>>,
    rh_link: prim::HandleLink,
) -> Result<(), Trap>
where
    S: Store,
{
    use dagwasm_handle::Handle;

    let link: Handle<LinkFor<S>> = rh_link.into_host();
    caller.data_mut().links_mut().remove(link)?;
    Ok(())
}
