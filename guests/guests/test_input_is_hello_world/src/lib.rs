use dagwasm_guest::prim::HandleLink;
use dagwasm_guest::{log, Link};

#[no_mangle]
pub extern "C" fn derive(planprim: HandleLink) -> HandleLink {
    let plan = unsafe { Link::wrap_handle(planprim) };

    let input = plan.open_directory().select_entry("input");
    log!("input: {input:?}");

    let bytes = input.open_file().read_to_vec();
    let contents = String::from_utf8_lossy(&bytes);
    log!("contents: {:?}", &contents);
    assert_eq!(contents, "Hello World!");

    unsafe { plan.unwrap_handle() }
}
