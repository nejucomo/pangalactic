use dagwasm_guest::{define_derive, Link, Plan};

#[define_derive]
fn derive_impl(plan: Plan) -> Link {
    plan.input
}
