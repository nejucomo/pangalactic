use pangalactic_store::define_standard_store_tests;

define_standard_store_tests!(crate::CryptoStore::from(
    pangalactic_dirstore::DirStore::init({
        use testdir::testdir;
        testdir!()
    })
));
