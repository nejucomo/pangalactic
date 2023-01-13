use dagwasm_guest::{define_derive, log, Link, Plan};

#[define_derive]
fn derive_impl(plan: Plan) -> Link {
    let bytes = plan.input.open_file().read_to_vec();
    let contents = String::from_utf8_lossy(&bytes);
    log!("contents: {:?}", &contents);
    assert_eq!(contents, "Hello World!");

    plan.input
}
