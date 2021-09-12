pub mod utils;
use utils::{LinkFor, MemStore, TestSetup};

def_test! {
    get_exec: |setup: TestSetup, outlink: LinkFor<MemStore>| {
        assert_eq!(&outlink, &setup.wasmlink);
        Ok(())
    }
}
