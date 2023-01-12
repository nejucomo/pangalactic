use dagwasm_guest::{define_derive, Link};

#[define_derive]
fn derive_impl(plan: Link) -> Link {
    plan
}
