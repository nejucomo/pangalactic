use dagwasm_dir::LinkKind;
use dagwasm_handle::Handle;

pub(crate) trait HostToWasm {
    fn into_wasm(self) -> u64;
}

pub(crate) trait WasmToHost<T> {
    fn into_host(self) -> T;
}

impl HostToWasm for bool {
    fn into_wasm(self) -> u64 {
        if self {
            1
        } else {
            0
        }
    }
}

impl HostToWasm for usize {
    fn into_wasm(self) -> u64 {
        u64::try_from(self).expect("HostToWasm usize->u64 failure")
    }
}

impl WasmToHost<usize> for u64 {
    fn into_host(self) -> usize {
        usize::try_from(self).expect("WasmToHost u64->usize failure")
    }
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

impl<T> HostToWasm for Handle<T> {
    fn into_wasm(self) -> u64 {
        unsafe { self.peek() }
    }
}

impl<T> WasmToHost<Handle<T>> for u64 {
    fn into_host(self) -> Handle<T> {
        unsafe { Handle::wrap(self) }
    }
}
