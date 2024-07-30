use crate::{ByteReader, DirectoryReader, State};
use pangalactic_handle::Handle;
use pangalactic_layer_cidmeta::CidMeta;
use pangalactic_link::Link;
use pangalactic_store::Store;
use wasmtime::{Caller, Trap};

pub(super) async fn has_more_entries<S>(
    caller: Caller<'_, State<S>>,
    h_dr: Handle<DirectoryReader<S>>,
) -> Result<bool, Trap>
where
    S: Store,
{
    let dr = caller.data().directory_readers().lookup(h_dr)?;
    Ok(dr.has_more_entries())
}

pub(super) async fn load_link<S>(
    mut caller: Caller<'_, State<S>>,
    h_dr: Handle<DirectoryReader<S>>,
) -> Result<Handle<Link<CidMeta<S::CID>>>, Trap>
where
    S: Store,
{
    let dr = caller.data_mut().directory_readers_mut().lookup_mut(h_dr)?;
    let link = dr.take_link()?;
    let h_link = caller.data_mut().links_mut().insert(link);
    Ok(h_link)
}

pub(super) async fn open_name_reader<S>(
    mut caller: Caller<'_, State<S>>,
    h_dr: Handle<DirectoryReader<S>>,
) -> Result<Handle<ByteReader<S>>, Trap>
where
    S: Store,
{
    let dr = caller.data_mut().directory_readers_mut().lookup_mut(h_dr)?;
    let name = dr.take_name()?;
    let br = ByteReader::from(name);
    let h_br = caller.data_mut().byte_readers_mut().insert(br);
    Ok(h_br)
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
    Ok(())
}

pub(super) async fn close<S>(
    mut caller: Caller<'_, State<S>>,
    h_dr: Handle<DirectoryReader<S>>,
) -> Result<(), Trap>
where
    S: Store,
{
    caller.data_mut().directory_readers_mut().remove(h_dr)?;
    Ok(())
}
