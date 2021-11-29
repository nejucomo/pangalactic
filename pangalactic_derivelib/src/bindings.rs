use crate::prim;

#[link(wasm_import_module = "pangalactic_bindings")]
extern "C" {
    // BufWriter methods:
    pub(crate) fn bufwriter_new() -> prim::BufWriterHandle;
    pub(crate) fn bufwriter_write(
        handle: prim::BufWriterHandle,
        dataptr: prim::ReadPtr,
        datalen: prim::MemLen,
    );
    pub(crate) fn bufwriter_commit(handle: prim::BufWriterHandle) -> prim::LinkHandle;

    // Link methods:
    pub(crate) fn link_kind(handle: prim::LinkHandle) -> prim::LinkKind;
    pub(crate) fn link_load_file(handle: prim::LinkHandle) -> prim::BufReaderHandle;
}
