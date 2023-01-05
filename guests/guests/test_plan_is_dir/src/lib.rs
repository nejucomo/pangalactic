use dagwasm_guest::{prim, Link, LinkKind};

#[no_mangle]
pub extern "C" fn derive(primplan: prim::HandleLink) -> prim::HandleLink {
    let plan = unsafe { Link::wrap_handle(primplan) };
    let kind = plan.kind();
    assert_eq!(kind, LinkKind::Dir);
    unsafe { plan.unwrap_handle() }
}
