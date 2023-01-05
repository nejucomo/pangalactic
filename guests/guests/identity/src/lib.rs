use dagwasm_guest::{fail, prim, Link};

#[no_mangle]
pub extern "C" fn derive(primplan: prim::HandleLink) -> prim::HandleLink {
    let plan = unsafe { Link::wrap_handle(primplan) };
    let output = derive_inner(plan);
    unsafe { output.unwrap_handle() }
}

fn derive_inner(plan: Link) -> Link {
    for (name, link) in plan.open_directory() {
        if name == "input" {
            return link;
        }
    }
    fail!("no 'input' found in directory")
}
