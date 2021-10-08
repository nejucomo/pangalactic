pub mod utils;
use utils::{LinkFor, MemStore, SetupLinks};

def_test! {
    get_exec: |slinks: SetupLinks, outlink: LinkFor<MemStore>| {
        assert_eq!(&outlink, &slinks.wasmlink);
        Ok(())
    }
}
