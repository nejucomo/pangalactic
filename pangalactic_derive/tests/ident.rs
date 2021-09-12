pub mod utils;
use utils::{LinkFor, MemStore, TestSetup};

def_test! {
    ident: |setup: TestSetup, outlink: LinkFor<MemStore>| {
        assert_eq!(&outlink, &setup.inputlink);
        Ok(())
    }
}
