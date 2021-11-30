use crate::vm::{BufReaderHandle, VirtualMachine};
use pangalactic_store::Store;
use wasmi::Trap;

pub(super) fn bufreader_read<S>(
    vm: &mut VirtualMachine<S>,
    brh: BufReaderHandle,
    dataptr: usize,
    datalen: usize,
) -> Result<usize, Trap>
where
    S: Store,
{
    let hostbuf = vm.brtab.get(brh)?;
    let len = std::cmp::min(datalen, hostbuf.len());

    vm.memory.with_direct_access_mut(|mem| {
        let guestbuf = &mut mem[dataptr..dataptr + len];
        guestbuf.clone_from_slice(&hostbuf[..len]);
        Ok(len)
    })
}
