use super::iotrap::IOTrap;
use crate::vm::{BufReaderHandle, LinkHandle, VirtualMachine};
use pangalactic_node::Kind;
use pangalactic_store::Store;
use wasmi::Trap;

pub(super) fn link_kind<S>(vm: &mut VirtualMachine<S>, handle: LinkHandle<S>) -> Result<Kind, Trap>
where
    S: Store,
{
    let link = vm.links.get(handle)?;
    Ok(link.kind)
}

pub(super) fn link_eq<S>(
    vm: &mut VirtualMachine<S>,
    a: LinkHandle<S>,
    b: LinkHandle<S>,
) -> Result<bool, Trap>
where
    S: Store,
{
    let linka = vm.links.get(a)?;
    let linkb = vm.links.get(b)?;
    Ok(linka == linkb)
}

pub(super) fn link_load_file<S>(
    vm: &mut VirtualMachine<S>,
    handle: LinkHandle<S>,
) -> Result<BufReaderHandle, IOTrap>
where
    S: Store,
{
    // TODO: Should we remove the link from the table?
    let link = vm.links.get(handle)?;
    let fkey = link.get_file_key()?;
    let bytes = vm.nodestore.get_file(fkey)?;
    Ok(vm.brtab.insert(bytes))
}
