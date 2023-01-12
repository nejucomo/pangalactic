mod byte_reader;
mod byte_writer;
mod directory_reader;
mod directory_writer;
mod link;

use crate::State;
use dagwasm_store::Store;
use wasmtime::{Caller, Engine, Linker, Trap};

pub(crate) fn instantiate_linker<S>(engine: &Engine) -> anyhow::Result<Linker<State<S>>>
where
    S: Store,
{
    use crate::{HostToWasm, WasmToHost};

    const HOSTMOD: &str = env!("CARGO_PKG_NAME");

    let mut linker = Linker::new(engine);

    macro_rules! link_host_fn {
        ( method $wrapmethod:ident, $modname:ident, $methodname:ident, $( $arg:ident ),* ) => {
            linker . $wrapmethod(
                HOSTMOD,
                &format!("{}_{}", stringify!($modname), stringify!($methodname)),
                |caller: Caller<'_, State<S>>, $( $arg : u64 ),* | Box::new(async move {
                    self::$modname::$methodname(caller, $( $arg.into_host() ),* ).await.into_wasm()
                }),
            )
        };

        ( $modname:ident, $methodname:ident ) => {
            linker.func_wrap0_async(
                HOSTMOD,
                &format!("{}_{}", stringify!($modname), stringify!($methodname)),
                |caller: Caller<'_, State<S>>| Box::new(async {
                    self::$modname::$methodname(caller).await.into_wasm()
                }),
            )
        };

        ( $modname:ident, $methodname:ident, $a0:ident ) => {
            link_host_fn!(method func_wrap1_async, $modname, $methodname, $a0)
        };

        ( $modname:ident, $methodname:ident, $a0:ident, $a1:ident ) => {
            link_host_fn!(method func_wrap2_async, $modname, $methodname, $a0, $a1)
        };

        ( $modname:ident, $methodname:ident, $a0:ident, $a1:ident, $a2:ident ) => {
            link_host_fn!(method func_wrap3_async, $modname, $methodname, $a0, $a1, $a2)
        };

        ( $modname:ident, $methodname:ident, $a0:ident, $a1:ident, $a2:ident, $a3:ident ) => {
            link_host_fn!(method func_wrap4_async, $modname, $methodname, $a0, $a1, $a2, $a3)
        }
    }

    // Method bindings should follow structure in `dagwasm_guest::bindings`:

    // Log:
    // Note "log" is the only hostapi that does not follow the `<type>_<method>` name convention,
    // so we do not use a macro:
    linker.func_wrap2_async(
        HOSTMOD,
        "log",
        |caller: Caller<'_, State<S>>, ptr: u64, len: u64| {
            Box::new(log(caller, ptr.into_host(), len.into_host()))
        },
    )?;

    // Link methods:
    link_host_fn!(link, get_kind, link)?;
    link_host_fn!(link, open_file_reader, link)?;
    link_host_fn!(link, open_directory_reader, link)?;
    link_host_fn!(link, close, link)?;

    // ByteReader methods:
    link_host_fn!(byte_reader, read, byte_reader, ptr, len)?;
    link_host_fn!(byte_reader, close, byte_reader)?;

    // DirectoryReader methods:
    link_host_fn!(directory_reader, has_more_entries, directory_reader)?;
    link_host_fn!(directory_reader, load_link, directory_reader)?;
    link_host_fn!(directory_reader, open_name_reader, directory_reader)?;
    link_host_fn!(directory_reader, next_entry, directory_reader)?;
    link_host_fn!(directory_reader, close, directory_reader)?;

    // ByteWriter methods:
    link_host_fn!(byte_writer, open)?;
    link_host_fn!(byte_writer, write, byte_writer, ptr, len)?;
    link_host_fn!(byte_writer, commit, byte_writer)?;

    // ByteWriter methods:
    link_host_fn!(directory_writer, open)?;
    link_host_fn!(
        directory_writer,
        insert,
        directory_writer,
        nameptr,
        namelen,
        link
    )?;
    link_host_fn!(directory_writer, commit, directory_writer)?;

    Ok(linker)
}

async fn log<S>(mut caller: Caller<'_, State<S>>, ptr: usize, len: usize) -> Result<(), Trap>
where
    S: Store,
{
    use crate::CallerIO;

    caller.peek_into_guest(ptr, len, crate::guest_log::bytes)?;
    Ok(())
}
