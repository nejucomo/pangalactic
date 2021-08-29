use crate::{Publisher, Subscription};

#[test]
fn generate_publisher_and_derive_subscription() {
    let p = Publisher::generate();
    let s = Subscription::from(p.clone());
    let (signpub, sboxkey) = s.expose_innards();
    assert_eq!(p.signpair.public, *signpub);
    assert_eq!(p.sboxkey, *sboxkey);
}
