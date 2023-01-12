use dagwasm_guest::{define_derive, Link};

#[define_derive]
fn derive_impl(plan: Link) -> Link {
    plan.open_directory().select_entry("input")
}
