use pangalactic_guest::{define_derive, Link, LinkKind};

#[define_derive]
fn derive_impl(plan: Link) -> Link {
    let kind = plan.kind();
    assert_eq!(kind, LinkKind::Dir);
    plan
}
