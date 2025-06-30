use crate::{
    store::{HostLink, HostWriter},
    State,
};
use anyhow::anyhow;
use pangalactic_handle::Handle;
use pangalactic_store::Store;
use wasmtime::{Caller, Trap};

pub(super) async fn open<S>(mut caller: Caller<'_, State<S>>) -> Result<Handle<HostWriter<S>>, Trap>
where
    S: Store,
{
    let writer = caller.data_mut().store_mut().open_writer().await?;
    let handle = caller.data_mut().byte_writers_mut().insert(writer);
    Ok(handle)
}

pub(super) async fn write<S>(
    mut caller: Caller<'_, State<S>>,
    h_bw: Handle<HostWriter<S>>,
    ptr: usize,
    len: usize,
) -> Result<(), Trap>
where
    S: Store,
{
    use crate::CallerIO;
    use tokio::io::AsyncWriteExt;

    let intermediate = caller.guest_bytes_to_vec(ptr, len)?;
    let writer = caller.data_mut().byte_writers_mut().lookup_mut(h_bw)?;

    writer
        .write_all(&intermediate)
        .await
        .map_err(|e| anyhow!("{e}"))?;

    Ok(())
}

pub(super) async fn commit<S>(
    mut caller: Caller<'_, State<S>>,
    h_bw: Handle<HostWriter<S>>,
) -> Result<Handle<HostLink<S::CID>>, Trap>
where
    S: Store,
{
    let w = caller.data_mut().byte_writers_mut().remove(h_bw)?;
    let link = caller.data_mut().store_mut().commit(w).await?;
    let h_link = caller.data_mut().links_mut().insert(link);
    Ok(h_link)
}
