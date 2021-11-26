use crate::prim;

#[link(wasm_import_module = "pangalactic_bindings")]
extern "C" {
    pub(crate) fn link_kind(handle: prim::Link) -> prim::LinkKind;
    pub(crate) fn load_file(handle: prim::Link) -> prim::Read;
}
