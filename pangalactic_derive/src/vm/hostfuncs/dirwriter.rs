use crate::vm::{DirWriterHandle, VirtualMachine};
use pangalactic_node::Dir;
use pangalactic_store::Store;
use wasmi::Trap;

pub(super) fn dirwriter_new<S>(vm: &mut VirtualMachine<S>) -> Result<DirWriterHandle<S>, Trap>
where
    S: Store,
{
    Ok(vm.dwtab.insert(Dir::new()))
}
