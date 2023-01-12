use dagwasm_guest::prim::HandleLink;

#[no_mangle]
pub extern "C" fn prim_derive_impl(plan: HandleLink) -> HandleLink {
    plan
}
