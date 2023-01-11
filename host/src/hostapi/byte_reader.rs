use crate::{HostToWasm, State, WasmToHost};
use dagwasm_primitives as prim;
use dagwasm_store::Store;
use wasmtime::{Caller, Trap};

pub(super) async fn read<S>(
    mut caller: Caller<'_, State<S>>,
    rh_br: prim::HandleByteReader,
    ptr: prim::PtrRead,
    len: prim::ByteLen,
) -> Result<prim::ByteLen, Trap>
where
    S: Store,
{
    use crate::ByteReader;
    use dagwasm_handle::Handle;
    use tokio::io::AsyncReadExt;

    let h_br: Handle<ByteReader<S>> = rh_br.into_host();
    let ptr: usize = ptr.into_host();
    let len: usize = len.into_host();

    let reader = caller.data_mut().byte_readers_mut().lookup_mut(h_br)?;
    // TODO: Use a fixed-length host controlled buffer instead of guest-provided len:
    let mut buf = vec![0; len];
    let mut readlen = 0;
    while readlen < len {
        let c = reader
            .read(&mut buf[readlen..])
            .await
            .map_err(|e| anyhow::Error::msg(e.to_string()))?;

        if c == 0 {
            break;
        }
        readlen += c;
    }
    assert!(readlen <= len);
    let mem = super::get_memory(&mut caller)?;
    mem.data_mut(&mut caller)[ptr..ptr + readlen].copy_from_slice(&buf[..readlen]);
    Ok(readlen).into_wasm()
}

pub(super) async fn close<S>(
    mut caller: Caller<'_, State<S>>,
    rh_br: prim::HandleByteReader,
) -> Result<(), Trap>
where
    S: Store,
{
    use crate::ByteReader;
    use dagwasm_handle::Handle;

    let h_br: Handle<ByteReader<S>> = rh_br.into_host();
    caller.data_mut().byte_readers_mut().remove(h_br)?;
    Ok(()).into_wasm()
}
