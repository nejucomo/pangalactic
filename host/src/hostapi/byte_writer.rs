use crate::State;
use pangalactic_dagio::{DagioLink, DagioWriter};
use pangalactic_handle::Handle;
use pangalactic_store::Store;
use wasmtime::{Caller, Trap};

pub(super) async fn open<S>(
    mut caller: Caller<'_, State<S>>,
) -> Result<Handle<DagioWriter<S>>, Trap>
where
    S: Store,
{
    let writer = caller.data_mut().dagio_mut().open_file_writer().await?;
    let handle = caller.data_mut().byte_writers_mut().insert(writer);
    Ok(handle)
}

pub(super) async fn write<S>(
    mut caller: Caller<'_, State<S>>,
    h_bw: Handle<DagioWriter<S>>,
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
    let mut buf = &intermediate[..];

    while !buf.is_empty() {
        let c = writer
            .write(buf)
            .await
            .map_err(|e| anyhow::Error::msg(e.to_string()))?;
        assert!(c > 0);
        buf = &buf[c..];
    }

    Ok(())
}

pub(super) async fn commit<S>(
    mut caller: Caller<'_, State<S>>,
    h_bw: Handle<DagioWriter<S>>,
) -> Result<Handle<DagioLink<S>>, Trap>
where
    S: Store,
{
    let w = caller.data_mut().byte_writers_mut().remove(h_bw)?;
    let link = caller.data_mut().dagio_mut().commit_file_writer(w).await?;
    let h_link = caller.data_mut().links_mut().insert(link);
    Ok(h_link)
}
