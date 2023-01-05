use dagwasm_guest::prim::HandleLink;
use dagwasm_guest::{fail, log, Link};

#[no_mangle]
pub extern "C" fn derive(planprim: HandleLink) -> HandleLink {
    let plan = unsafe { Link::wrap_handle(planprim) };

    let input = get_input_link(&plan);
    log!("input: {:?}", input);

    let reader = input.open_file();
    let contents = reader.read_to_vec();
    assert_eq!(contents[..], b"Hello World!"[..]);
    unsafe { plan.unwrap_handle() }
}

fn get_input_link(plan: &Link) -> Link {
    let reader = plan.open_directory();
    for (name, link) in reader {
        if &name == "input" {
            return link;
        }
    }
    fail!("No `input` link found.");
}
