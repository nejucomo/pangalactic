use crate::{HostToWasm, State, WasmToHost};
use dagwasm_dagio::LinkFor;
use dagwasm_primitives as prim;
use dagwasm_store::Store;
use wasmtime::{Caller, Engine, Linker, Memory, Trap};

pub(crate) fn instantiate_linker<S>(engine: &Engine) -> anyhow::Result<Linker<State<S>>>
where
    S: Store,
{
    const HOSTMOD: &str = env!("CARGO_PKG_NAME");

    let mut linker = Linker::new(engine);

    macro_rules! link_host_fn {
        ( method $wrapmethod:ident, $name:ident, $( $arg:ident ),* ) => {
            linker . $wrapmethod(
                HOSTMOD,
                stringify!($name),
                |caller: Caller<'_, State<S>>, $( $arg : u64 ),* | Box::new($name(caller, $( $arg ),* )),
            )
        };

        ( $name:ident, $a0:ident ) => {
            link_host_fn!(method func_wrap1_async, $name, $a0)
        };

        ( $name:ident, $a0:ident, $a1:ident ) => {
            link_host_fn!(method func_wrap2_async, $name, $a0, $a1)
        };

        ( $name:ident, $a0:ident, $a1:ident, $a2:ident ) => {
            link_host_fn!(method func_wrap3_async, $name, $a0, $a1, $a2)
        }
    }

    // Method bindings should follow structure in `dagwasm_guest::bindings`:
    // Log:
    link_host_fn!(log, ptr, len)?;

    // Link methods:
    link_host_fn!(link_get_kind, link)?;
    link_host_fn!(link_open_file_reader, link)?;
    link_host_fn!(link_open_directory_reader, link)?;
    link_host_fn!(link_close, link)?;

    // ByteReader methods:
    link_host_fn!(byte_reader_read, byte_reader, ptr, len)?;
    link_host_fn!(byte_reader_close, byte_reader)?;

    // DirectoryReader methods:
    link_host_fn!(directory_reader_has_more_entries, directory_reader)?;
    link_host_fn!(directory_reader_load_link, directory_reader)?;
    link_host_fn!(directory_reader_open_name_reader, directory_reader)?;
    link_host_fn!(directory_reader_next_entry, directory_reader)?;
    link_host_fn!(directory_reader_close, directory_reader)?;

    Ok(linker)
}

async fn log<S>(
    mut caller: Caller<'_, State<S>>,
    ptr: prim::PtrWrite,
    len: prim::ByteLen,
) -> Result<(), Trap>
where
    S: Store,
{
    let ptr: usize = ptr.into_host();
    let len: usize = len.into_host();

    let mem = get_memory(&mut caller)?;
    crate::guest_log::bytes(&mem.data(&caller)[ptr..ptr + len]);
    Ok(())
}

async fn link_get_kind<S>(
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

async fn link_open_file_reader<S>(
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

async fn link_open_directory_reader<S>(
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

async fn link_close<S>(
    mut caller: Caller<'_, State<S>>,
    rh_link: prim::HandleLink,
) -> Result<(), Trap>
where
    S: Store,
{
    use dagwasm_handle::Handle;

    let link: Handle<LinkFor<S>> = rh_link.into_host();
    caller.data_mut().links_mut().close(link)?;
    Ok(())
}

async fn byte_reader_read<S>(
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
    let mem = get_memory(&mut caller)?;
    mem.data_mut(&mut caller)[ptr..ptr + readlen].copy_from_slice(&buf[..readlen]);
    Ok(readlen.into_wasm())
}

async fn byte_reader_close<S>(
    mut caller: Caller<'_, State<S>>,
    rh_br: prim::HandleByteReader,
) -> Result<(), Trap>
where
    S: Store,
{
    use crate::ByteReader;
    use dagwasm_handle::Handle;

    let h_br: Handle<ByteReader<S>> = rh_br.into_host();
    caller.data_mut().byte_readers_mut().close(h_br)?;
    Ok(())
}

async fn directory_reader_has_more_entries<S>(
    caller: Caller<'_, State<S>>,
    rh_dr: prim::HandleDirReader,
) -> Result<prim::Bool, Trap>
where
    S: Store,
{
    use crate::DirectoryReader;
    use dagwasm_handle::Handle;

    let h_dr: Handle<DirectoryReader<S>> = rh_dr.into_host();
    let dr = caller.data().directory_readers().lookup(h_dr)?;
    Ok(dr.has_more_entries().into_wasm())
}

async fn directory_reader_load_link<S>(
    mut caller: Caller<'_, State<S>>,
    rh_dr: prim::HandleDirReader,
) -> Result<prim::HandleLink, Trap>
where
    S: Store,
{
    use crate::DirectoryReader;
    use dagwasm_handle::Handle;

    let h_dr: Handle<DirectoryReader<S>> = rh_dr.into_host();
    let dr = caller.data_mut().directory_readers_mut().lookup_mut(h_dr)?;
    let link = dr.take_link()?;
    let h_link = caller.data_mut().links_mut().insert(link);
    Ok(h_link.into_wasm())
}

async fn directory_reader_open_name_reader<S>(
    mut caller: Caller<'_, State<S>>,
    rh_dr: prim::HandleDirReader,
) -> Result<prim::HandleByteReader, Trap>
where
    S: Store,
{
    use crate::{ByteReader, DirectoryReader};
    use dagwasm_handle::Handle;

    let h_dr: Handle<DirectoryReader<S>> = rh_dr.into_host();
    let dr = caller.data_mut().directory_readers_mut().lookup_mut(h_dr)?;
    let name = dr.take_name()?;
    let br = ByteReader::from(name);
    let h_br = caller.data_mut().byte_readers_mut().insert(br);
    Ok(h_br.into_wasm())
}

async fn directory_reader_next_entry<S>(
    mut caller: Caller<'_, State<S>>,
    rh_dr: prim::HandleDirReader,
) -> Result<(), Trap>
where
    S: Store,
{
    use crate::DirectoryReader;
    use dagwasm_handle::Handle;

    let h_dr: Handle<DirectoryReader<S>> = rh_dr.into_host();
    let dr = caller.data_mut().directory_readers_mut().lookup_mut(h_dr)?;
    dr.next_entry();
    Ok(())
}

async fn directory_reader_close<S>(
    mut caller: Caller<'_, State<S>>,
    rh_dr: prim::HandleDirReader,
) -> Result<(), Trap>
where
    S: Store,
{
    use crate::DirectoryReader;
    use dagwasm_handle::Handle;

    let h_dr: Handle<DirectoryReader<S>> = rh_dr.into_host();
    caller.data_mut().directory_readers_mut().close(h_dr)?;
    Ok(())
}

fn get_memory<S>(caller: &mut Caller<'_, State<S>>) -> anyhow::Result<Memory>
where
    S: Store,
{
    use wasmtime::Extern::*;

    let export = caller
        .get_export("memory")
        .ok_or_else(|| anyhow::Error::msg("no 'memory' export found"))?;

    match export {
        Memory(m) => Ok(m),
        _ => Err(anyhow::Error::msg(
            "the 'memory' export is not a wasmtime::Memory",
        )),
    }
}
