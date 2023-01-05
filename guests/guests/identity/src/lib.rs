use dagwasm_guest::{prim, Link};

#[no_mangle]
pub extern "C" fn derive(primplan: prim::HandleLink) -> prim::HandleLink {
    let plan = unsafe { Link::wrap_handle(primplan) };
    let output = plan.open_directory().select_entry("input");
    unsafe { output.unwrap_handle() }
}
