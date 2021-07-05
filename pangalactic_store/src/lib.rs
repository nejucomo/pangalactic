mod store;

pub use store::{ReadVerify, Store, StoreKey, WriteCommit};

pub fn test_store_then_read_then_store<S>(store: S) -> std::io::Result<()>
where
    S: Store,
{
    let cases: &[&[u8]] = &[b"", b"hello world"];
    for &contents in cases {
        let key = store.write(contents)?;
        let c2 = store.read(&key)?;
        assert_eq!(&c2[..], contents);
        let k2 = store.write(contents)?;
        assert!(k2 == key, "{:?} != {:?}", k2.b64_encode(), key.b64_encode());
    }
    Ok(())
}
