use dagwasm_guest::prim::HandleLink;

#[no_mangle]
pub extern "C" fn derive(plan: HandleLink) -> HandleLink {
    plan
}
