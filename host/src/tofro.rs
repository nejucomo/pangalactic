use dagwasm_dir::LinkKind;
use dagwasm_handle::Handle;

pub(crate) trait HostToWasm {
    fn into_wasm(self) -> u64;
}

pub(crate) trait WasmToHost<T> {
    fn into_host(self) -> T;
}

impl HostToWasm for LinkKind {
    fn into_wasm(self) -> u64 {
        use LinkKind::*;

        match self {
            File => 0,
            Dir => 1,
        }
    }
}

impl<T> WasmToHost<Handle<T>> for u64 {
    fn into_host(self) -> Handle<T> {
        unsafe { Handle::wrap(self) }
    }
}
