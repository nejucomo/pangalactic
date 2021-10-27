use crate::vm::{LinkHandle, ReadHandle, VirtualMachine};
use pangalactic_node::Kind;
use pangalactic_store::Store;
use pangalactic_wasmi::HostFuncResolver;
use wasmi::Trap;

pub fn new_hostfunc_resolver<S>() -> HostFuncResolver<VirtualMachine<S>>
where
    S: Store + 'static,
{
    let mut hfr = HostFuncResolver::new();
    hfr.add_host_fn1(link_kind);
    hfr.add_host_fn1(open_file);
    hfr
}

fn link_kind<S>(vm: &mut VirtualMachine<S>, handle: LinkHandle<S>) -> Result<Kind, Trap>
where
    S: Store,
{
    let link = vm.links.get(handle)?;
    Ok(link.kind)
}

fn open_file<S>(vm: &mut VirtualMachine<S>, handle: LinkHandle<S>) -> Result<ReadHandle, Trap>
where
    S: Store,
{
    // BUG: This is an expedient hack for std::io::Errors:
    let mktrap = |_| Trap::new(wasmi::TrapKind::Unreachable);

    let link = vm.links.get(handle)?;
    let fkey = link.get_file_key().map_err(mktrap)?;
    let bytes = vm.nodestore.get_file(fkey).map_err(mktrap)?;
    Ok(vm.readtab.append(bytes))
}
