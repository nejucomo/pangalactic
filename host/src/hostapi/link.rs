use crate::{ByteReader, DirectoryReader, State};
use pangalactic_dagio::DagioLink;
use pangalactic_handle::Handle;
use pangalactic_linkkind::LinkKind;
use pangalactic_store::Store;
use wasmtime::{Caller, Trap};

pub(super) async fn get_kind<S>(
    caller: Caller<'_, State<S>>,
    h_link: Handle<DagioLink<S>>,
) -> Result<LinkKind, Trap>
where
    S: Store,
{
    let link = caller.data().links().lookup(h_link)?;
    Ok(link.kind())
}

pub(super) async fn node_size<S>(
    caller: Caller<'_, State<S>>,
    h_link: Handle<DagioLink<S>>,
) -> Result<u64, Trap>
where
    S: Store,
{
    let link = caller.data().links().lookup(h_link)?;
    Ok(link.peek_key().node_size())
}

pub(super) async fn open_file_reader<S>(
    mut caller: Caller<'_, State<S>>,
    h_link: Handle<DagioLink<S>>,
) -> Result<Handle<ByteReader<S>>, Trap>
where
    S: Store,
{
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

    Ok(h_fr)
}

pub(super) async fn open_directory_reader<S>(
    mut caller: Caller<'_, State<S>>,
    h_link: Handle<DagioLink<S>>,
) -> Result<Handle<DirectoryReader<S>>, Trap>
where
    S: Store,
{
    let link = caller.data().links().lookup(h_link)?.clone();
    let dr: DirectoryReader<S> = caller.data_mut().dagio_mut().load(&link).await?;
    let h_dr = caller.data_mut().directory_readers_mut().insert(dr);
    Ok(h_dr)
}

pub(super) async fn close<S>(
    mut caller: Caller<'_, State<S>>,
    link: Handle<DagioLink<S>>,
) -> Result<(), Trap>
where
    S: Store,
{
    caller.data_mut().links_mut().remove(link)?;
    Ok(())
}
