use crate::{HostToWasm, State, WasmToHost};
use dagwasm_primitives as prim;
use dagwasm_store::Store;
use wasmtime::{Caller, Trap};

pub(super) async fn open<S>(
    mut caller: Caller<'_, State<S>>,
) -> Result<prim::HandleByteWriter, Trap>
where
    S: Store,
{
    let writer = caller.data_mut().dagio_mut().open_file_writer().await?;
    let handle = caller.data_mut().byte_writers_mut().insert(writer);
    Ok(handle.into_wasm())
}

pub(super) async fn write<S>(
    mut caller: Caller<'_, State<S>>,
    rh_bw: prim::HandleByteWriter,
    ptr: prim::PtrWrite,
    len: prim::ByteLen,
) -> Result<(), Trap>
where
    S: Store,
{
    use dagwasm_handle::Handle;
    use tokio::io::AsyncWriteExt;

    let h_bw: Handle<<S as Store>::Writer> = rh_bw.into_host();

    let ptr: usize = ptr.into_host();
    let len: usize = len.into_host();

    let intermediate = {
        let mut buf = vec![0; len]; // FIXME: don't allocate on guest-provided `len`.
        let mem = super::get_memory(&mut caller)?;
        buf.copy_from_slice(&mem.data(&caller)[ptr..ptr + len]);
        buf
    };

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
    rh_bw: prim::HandleByteWriter,
) -> Result<prim::HandleLink, Trap>
where
    S: Store,
{
    use dagwasm_handle::Handle;

    let h_bw: Handle<<S as Store>::Writer> = rh_bw.into_host();

    let w = caller.data_mut().byte_writers_mut().remove(h_bw)?;
    let link = caller.data_mut().dagio_mut().commit_file_writer(w).await?;
    let h_link = caller.data_mut().links_mut().insert(link);
    Ok(h_link.into_wasm())
}
