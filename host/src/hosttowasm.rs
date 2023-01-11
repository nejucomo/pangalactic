use dagwasm_handle::Handle;
use dagwasm_linkkind::LinkKind;

pub(crate) trait HostToWasm {
    fn into_wasm(self) -> u64;
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
