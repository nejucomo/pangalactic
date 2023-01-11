use dagwasm_handle::Handle;

pub(crate) trait WasmToHost<T> {
    fn into_host(self) -> T;
}

impl WasmToHost<usize> for u64 {
    fn into_host(self) -> usize {
        usize::try_from(self).expect("WasmToHost u64->usize failure")
    }
}

impl<T> WasmToHost<Handle<T>> for u64 {
    fn into_host(self) -> Handle<T> {
        unsafe { Handle::wrap(self) }
    }
}
