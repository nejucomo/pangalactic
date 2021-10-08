pub mod utils;
use utils::{LinkFor, MemStore, SetupLinks};

def_test! {
    ident: |slinks: SetupLinks, outlink: LinkFor<MemStore>| {
        assert_eq!(&outlink, &slinks.inputlink);
        Ok(())
    }
}
