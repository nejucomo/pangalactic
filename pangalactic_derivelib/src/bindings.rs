#[link(wasm_import_module = "pangalactic_bindings")]
extern "C" {
    pub(crate) fn link_kind(handle: i64) -> i64;
}
