mod bytes {
    mod positive {
        #[test]
        fn enc_dec() {
            use crate::{decode_bytes, encode_bytes};

            let payload: (u64, &str) = (42, "Hello World");
            let bytes = encode_bytes(&payload);
            let p2 = decode_bytes(&bytes[..]).unwrap();
            assert_eq!(payload, p2);
        }
    }
}
