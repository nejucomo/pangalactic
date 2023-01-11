use dagwasm_handle::Handle;
use dagwasm_linkkind::LinkKind;
use wasmtime::{Trap, WasmRet};

pub(crate) trait HostToWasm {
    type WasmTy: WasmRet;

    fn into_wasm(self) -> Self::WasmTy;
}

impl<T> HostToWasm for Result<T, Trap>
where
    T: HostToWasm,
{
    type WasmTy = Result<<T as HostToWasm>::WasmTy, Trap>;

    fn into_wasm(self) -> Self::WasmTy {
        self.map(|v| v.into_wasm())
    }
}

impl HostToWasm for () {
    type WasmTy = ();

    fn into_wasm(self) {}
}

impl HostToWasm for bool {
    type WasmTy = u64;

    fn into_wasm(self) -> u64 {
        if self {
            1
        } else {
            0
        }
    }
}

impl HostToWasm for usize {
    type WasmTy = u64;

    fn into_wasm(self) -> u64 {
        u64::try_from(self).expect("HostToWasm usize->u64 failure")
    }
}

impl HostToWasm for LinkKind {
    type WasmTy = u64;

    fn into_wasm(self) -> u64 {
        use LinkKind::*;

        match self {
            File => 0,
            Dir => 1,
        }
    }
}

impl<T> HostToWasm for Handle<T> {
    type WasmTy = u64;

    fn into_wasm(self) -> u64 {
        unsafe { self.peek() }
    }
}
