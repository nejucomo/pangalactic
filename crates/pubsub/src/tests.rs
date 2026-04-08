use pangalactic_serialization::{deserialize, serialize};

use crate::{History, PublishCap, Subscription};

#[test]
fn round_trip_initial() {
    let pubcap = PublishCap::generate(rand::rng());
    let subcap = pubcap.subscribe_cap();

    let envelope = pubcap
        .publish(History::Initial { content: 42u64 })
        .unwrap();

    let bytes = serialize(&envelope).unwrap();
    let envelope2: crate::Envelope = deserialize(&bytes).unwrap();
    let sub: Subscription<u64> = envelope2.try_into().unwrap();

    assert!(matches!(
        sub.history(),
        Some(History::Initial { content: 42 })
    ));
    assert_eq!(
        sub.subcap().verifying_key_bytes(),
        subcap.verifying_key_bytes()
    );
}

#[test]
fn round_trip_subsequent() {
    let pubcap = PublishCap::generate(rand::rng());

    let envelope = pubcap
        .publish(History::Subsequent {
            content: String::from("v2"),
            prev: String::from("v1"),
        })
        .unwrap();

    let bytes = serialize(&envelope).unwrap();
    let envelope2: crate::Envelope = deserialize(&bytes).unwrap();
    let sub: Subscription<String> = envelope2.try_into().unwrap();

    assert!(matches!(
        sub.history(),
        Some(History::Subsequent { content, prev }) if content == "v2" && prev == "v1"
    ));
}

#[test]
fn different_keys_each_verify_independently() {
    let pubcap = PublishCap::generate(rand::rng());
    let other_pubcap = PublishCap::generate(rand::rng());

    // Envelope signed by `pubcap`
    let env_a = pubcap
        .publish(History::Initial { content: 1u8 })
        .unwrap();

    // Envelope signed by `other_pubcap` — subcap inside refers to other_pubcap's key
    let env_b = other_pubcap
        .publish(History::Initial { content: 1u8 })
        .unwrap();

    // Each envelope should verify against its own key
    let bytes_a = serialize(&env_a).unwrap();
    let bytes_b = serialize(&env_b).unwrap();
    let env_a2: crate::Envelope = deserialize(&bytes_a).unwrap();
    let env_b2: crate::Envelope = deserialize(&bytes_b).unwrap();
    let _: Subscription<u8> = env_a2.try_into().unwrap();
    let _: Subscription<u8> = env_b2.try_into().unwrap();
}

#[test]
fn tampered_signature_fails_verification() {
    let pubcap = PublishCap::generate(rand::rng());
    let env = pubcap
        .publish(History::Initial { content: 42u64 })
        .unwrap();

    // Flip the last byte of the serialized form, which falls inside the 64-byte signature
    let mut bytes = serialize(&env).unwrap();
    let last = bytes.len() - 1;
    bytes[last] ^= 0xff;

    // Either deserialization or signature verification must fail
    match deserialize::<_, crate::Envelope>(&bytes) {
        Err(_) => {}
        Ok(tampered_env) => {
            let result: anyhow::Result<Subscription<u64>> = tampered_env.try_into();
            assert!(result.is_err(), "tampered signature should fail verification");
        }
    }
}
