use crate::DirectoryReader;
use crate::{HostToWasm, State};
use dagwasm_handle::Handle;
use dagwasm_primitives as prim;
use dagwasm_store::Store;
use wasmtime::{Caller, Trap};

pub(super) async fn has_more_entries<S>(
    caller: Caller<'_, State<S>>,
    h_dr: Handle<DirectoryReader<S>>,
) -> Result<prim::Bool, Trap>
where
    S: Store,
{
    let dr = caller.data().directory_readers().lookup(h_dr)?;
    Ok(dr.has_more_entries()).into_wasm()
}

pub(super) async fn load_link<S>(
    mut caller: Caller<'_, State<S>>,
    h_dr: Handle<DirectoryReader<S>>,
) -> Result<prim::HandleLink, Trap>
where
    S: Store,
{
    let dr = caller.data_mut().directory_readers_mut().lookup_mut(h_dr)?;
    let link = dr.take_link()?;
    let h_link = caller.data_mut().links_mut().insert(link);
    Ok(h_link).into_wasm()
}

pub(super) async fn open_name_reader<S>(
    mut caller: Caller<'_, State<S>>,
    h_dr: Handle<DirectoryReader<S>>,
) -> Result<prim::HandleByteReader, Trap>
where
    S: Store,
{
    use crate::ByteReader;

    let dr = caller.data_mut().directory_readers_mut().lookup_mut(h_dr)?;
    let name = dr.take_name()?;
    let br = ByteReader::from(name);
    let h_br = caller.data_mut().byte_readers_mut().insert(br);
    Ok(h_br).into_wasm()
}

pub(super) async fn next_entry<S>(
    mut caller: Caller<'_, State<S>>,
    h_dr: Handle<DirectoryReader<S>>,
) -> Result<(), Trap>
where
    S: Store,
{
    let dr = caller.data_mut().directory_readers_mut().lookup_mut(h_dr)?;
    dr.next_entry();
    Ok(()).into_wasm()
}

pub(super) async fn close<S>(
    mut caller: Caller<'_, State<S>>,
    h_dr: Handle<DirectoryReader<S>>,
) -> Result<(), Trap>
where
    S: Store,
{
    caller.data_mut().directory_readers_mut().remove(h_dr)?;
    Ok(()).into_wasm()
}
