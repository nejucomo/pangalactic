use dagwasm_guest::prim::HandleLink;
use dagwasm_guest::{
    Link,
    Reader::{Dir, File},
};

#[no_mangle]
pub extern "C" fn derive(planprim: HandleLink) -> HandleLink {
    let plan = unsafe { Link::wrap_handle(planprim) };

    let input = get_input_link(&plan);

    if let File(reader) = input.open() {
        let contents = reader.read_to_vec();
        assert_eq!(contents[..], b"Hello World!"[..]);
        unsafe { plan.unwrap_handle() }
    } else {
        panic!("input was not a file");
    }
}

fn get_input_link(plan: &Link) -> Link {
    if let Dir(reader) = plan.open() {
        for (name, link) in reader {
            if &name == "input" {
                return link;
            }
        }
        panic!("No `input` link found.");
    } else {
        panic!("plan was not a directory.");
    }
}
