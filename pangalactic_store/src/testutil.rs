#[macro_export]
macro_rules! define_standard_store_tests {
    ( $mkstore:expr ) => {
        #[cfg(test)]
        mod standard_store_tests {
            $crate::define_standard_store_then_read_then_store_test!(
                $mkstore,
                test_store_then_read_then_store_empty,
                b""
            );

            $crate::define_standard_store_then_read_then_store_test!(
                $mkstore,
                test_store_then_read_then_store_hello_world,
                b"hello world"
            );
        }
    };
}

#[macro_export]
macro_rules! define_standard_store_then_read_then_store_test {
    ( $mkstore:expr, $name:ident, $contents:expr ) => {
        #[test]
        fn $name() -> std::io::Result<()> {
            use pangalactic_store::{Store, StoreKey};

            let mut store = $mkstore;
            let contents = $contents;
            let key = store.write(contents)?;
            let c2 = store.read(&key)?;
            assert_eq!(&c2[..], contents);
            let k2 = store.write(contents)?;
            assert!(k2 == key, "{:?} != {:?}", k2.b64_encode(), key.b64_encode());
            Ok(())
        }
    };
}
