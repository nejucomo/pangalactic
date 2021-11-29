mod iotrap;

use self::iotrap::IOTrap;
use crate::vm::{BufWriterHandle, LinkHandle, ReadHandle, VirtualMachine};
use pangalactic_node::Kind;
use pangalactic_store::Store;
use pangalactic_wasmi::{HostFuncResolver, Void};
use wasmi::Trap;

pub fn new_hostfunc_resolver<S>() -> HostFuncResolver<VirtualMachine<S>>
where
    S: Store + 'static,
{
    let mut hfr = HostFuncResolver::new();
    hfr.add_host_fn0(new_file);
    hfr.add_host_fn3(bufwriter_write);
    hfr.add_host_fn1(bufwriter_commit);
    hfr.add_host_fn1(link_kind);
    hfr.add_host_fn1(load_file);
    log::debug!("Instantiated derive resolver: {:#?}", &hfr);
    hfr
}

fn new_file<S>(vm: &mut VirtualMachine<S>) -> Result<BufWriterHandle, Trap>
where
    S: Store,
{
    Ok(vm.bwtab.insert(vec![]))
}

fn bufwriter_write<S>(
    vm: &mut VirtualMachine<S>,
    bwh: BufWriterHandle,
    dataptr: usize,
    datalen: usize,
) -> Result<Void, Trap>
where
    S: Store,
{
    let hostbuf = vm.bwtab.get_mut(bwh)?;

    vm.memory.with_direct_access(|mem| {
        let guestbuf = &mem[dataptr..dataptr + datalen];
        hostbuf.extend_from_slice(guestbuf);
        Ok(Void)
    })
}

fn bufwriter_commit<S>(
    vm: &mut VirtualMachine<S>,
    bwh: BufWriterHandle,
) -> Result<LinkHandle<S>, IOTrap>
where
    S: Store,
{
    let buf = vm.bwtab.remove(bwh)?;
    let link = vm.nodestore.put_file(buf)?;
    let linkhandle = vm.links.insert(link);
    Ok(linkhandle)
}

fn link_kind<S>(vm: &mut VirtualMachine<S>, handle: LinkHandle<S>) -> Result<Kind, Trap>
where
    S: Store,
{
    let link = vm.links.get(handle)?;
    Ok(link.kind)
}

fn load_file<S>(vm: &mut VirtualMachine<S>, handle: LinkHandle<S>) -> Result<ReadHandle, IOTrap>
where
    S: Store,
{
    let link = vm.links.get(handle)?;
    let fkey = link.get_file_key()?;
    let bytes = vm.nodestore.get_file(fkey)?;
    Ok(vm.readtab.insert(bytes))
}
