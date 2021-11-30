use super::iotrap::IOTrap;
use crate::vm::{BufWriterHandle, LinkHandle, VirtualMachine};
use pangalactic_store::Store;
use pangalactic_wasmi::Void;
use wasmi::Trap;

pub(super) fn bufwriter_new<S>(vm: &mut VirtualMachine<S>) -> Result<BufWriterHandle, Trap>
where
    S: Store,
{
    Ok(vm.bwtab.insert(vec![]))
}

pub(super) fn bufwriter_write<S>(
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

pub(super) fn bufwriter_commit<S>(
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
