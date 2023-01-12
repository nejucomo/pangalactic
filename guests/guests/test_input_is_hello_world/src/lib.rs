use dagwasm_guest::{define_derive, log, Link};

#[define_derive]
fn derive_impl(plan: Link) -> Link {
    let input = plan.open_directory().select_entry("input");
    log!("input: {input:?}");

    let bytes = input.open_file().read_to_vec();
    let contents = String::from_utf8_lossy(&bytes);
    log!("contents: {:?}", &contents);
    assert_eq!(contents, "Hello World!");

    plan
}
