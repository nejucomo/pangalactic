#[link(wasm_import_module = "pangalactic_bindings")]
extern "C" {
    pub(crate) fn link_type(handle: i64) -> i64;
}
