use crate::{ByteReader, DirectoryReader, State};
use pangalactic_handle::Handle;
use pangalactic_layer_cidmeta::CidMeta;
use pangalactic_link::Link;
use pangalactic_linkkind::LinkKind;
use pangalactic_store::Store;
use wasmtime::{Caller, Trap};

pub(super) async fn get_kind<S>(
    caller: Caller<'_, State<S>>,
    h_link: Handle<Link<CidMeta<S::CID>>>,
) -> Result<LinkKind, Trap>
where
    S: Store,
{
    let link = caller.data().links().lookup(h_link)?;
    Ok(link.kind())
}

pub(super) async fn node_size<S>(
    caller: Caller<'_, State<S>>,
    h_link: Handle<Link<CidMeta<S::CID>>>,
) -> Result<u64, Trap>
where
    S: Store,
{
    let link = caller.data().links().lookup(h_link)?;
    Ok(link.peek_cid().node_size())
}

pub(super) async fn open_file_reader<S>(
    mut caller: Caller<'_, State<S>>,
    h_link: Handle<Link<CidMeta<S::CID>>>,
) -> Result<Handle<ByteReader<S>>, Trap>
where
    S: Store,
{
    use pangalactic_dagio::DagioReader;

    let link = caller.data().links().lookup(h_link)?.clone();

    let fr: DagioReader<_> = caller.data_mut().dagio_mut().load(&link).await?;

    let h_fr = caller
        .data_mut()
        .byte_readers_mut()
        .insert(ByteReader::Store(fr));

    Ok(h_fr)
}

pub(super) async fn open_directory_reader<S>(
    mut caller: Caller<'_, State<S>>,
    h_link: Handle<Link<CidMeta<S::CID>>>,
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
    link: Handle<Link<CidMeta<S::CID>>>,
) -> Result<(), Trap>
where
    S: Store,
{
    caller.data_mut().links_mut().remove(link)?;
    Ok(())
}
