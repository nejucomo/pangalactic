use crate::prim;

#[link(wasm_import_module = "pangalactic_bindings")]
extern "C" {
    pub(crate) fn new_file() -> prim::BufWriterHandle;
    pub(crate) fn bufwriter_write(
        handle: prim::BufWriterHandle,
        dataptr: prim::ReadPtr,
        datalen: prim::MemLen,
    );

    pub(crate) fn link_kind(handle: prim::LinkHandle) -> prim::LinkKind;
    pub(crate) fn load_file(handle: prim::LinkHandle) -> prim::BufReaderHandle;
}
