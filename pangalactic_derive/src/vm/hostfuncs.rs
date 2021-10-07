mod linkkind;

use crate::vm::VirtualMachine;
use pangalactic_store::Store;
use pangalactic_wasmi::HostFuncResolver;

pub fn new_hostfunc_resolver<'a, S>() -> HostFuncResolver<VirtualMachine<'a, S>>
where
    S: Store + 'static,
{
    let mut hfr = HostFuncResolver::new();
    hfr.add_host_func(self::linkkind::LinkKind::<S>::new());
    hfr
}
