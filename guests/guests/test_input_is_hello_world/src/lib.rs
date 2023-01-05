use dagwasm_guest::prim::HandleLink;
use dagwasm_guest::{
    fail, log, Link,
    Reader::{Dir, File},
};

#[no_mangle]
pub extern "C" fn derive(planprim: HandleLink) -> HandleLink {
    let plan = unsafe { Link::wrap_handle(planprim) };

    let input = get_input_link(&plan);
    log!("input: {:?}", input);

    if let File(reader) = input.open() {
        let contents = reader.read_to_vec();
        assert_eq!(contents[..], b"Hello World!"[..]);
        unsafe { plan.unwrap_handle() }
    } else {
        fail!("input {:?} was not a file", input);
    }
}

fn get_input_link(plan: &Link) -> Link {
    if let Dir(reader) = plan.open() {
        for (name, link) in reader {
            if &name == "input" {
                return link;
            }
        }
        fail!("No `input` link found.");
    } else {
        fail!("plan {:?} was not a directory.", plan);
    }
}
