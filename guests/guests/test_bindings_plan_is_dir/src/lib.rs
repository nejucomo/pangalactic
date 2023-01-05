use dagwasm_guest::bindings::link_get_kind;
use dagwasm_guest::prim::{HandleLink, LINK_KIND_DIR};

#[no_mangle]
pub extern "C" fn derive(plan: HandleLink) -> HandleLink {
    let kind = unsafe { link_get_kind(plan) };
    assert_eq!(kind, LINK_KIND_DIR);
    0
}
