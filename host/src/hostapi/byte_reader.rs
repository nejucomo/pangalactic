use crate::ByteReader;
use crate::{HostToWasm, State};
use dagwasm_handle::Handle;
use dagwasm_primitives as prim;
use dagwasm_store::Store;
use wasmtime::{Caller, Trap};

pub(super) async fn read<S>(
    mut caller: Caller<'_, State<S>>,
    h_br: Handle<ByteReader<S>>,
    ptr: usize,
    len: usize,
) -> Result<prim::ByteLen, Trap>
where
    S: Store,
{
    use tokio::io::AsyncReadExt;

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
    h_br: Handle<ByteReader<S>>,
) -> Result<(), Trap>
where
    S: Store,
{
    caller.data_mut().byte_readers_mut().remove(h_br)?;
    Ok(()).into_wasm()
}
