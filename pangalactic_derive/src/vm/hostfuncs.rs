mod iotrap;

use self::iotrap::IOTrap;
use crate::vm::{BufReaderHandle, BufWriterHandle, LinkHandle, VirtualMachine};
use pangalactic_node::Kind;
use pangalactic_store::Store;
use pangalactic_wasmi::{HostFuncResolver, Void};
use wasmi::Trap;

pub fn new_hostfunc_resolver<S>() -> HostFuncResolver<VirtualMachine<S>>
where
    S: Store + 'static,
{
    let mut hfr = HostFuncResolver::new();
    // BufWriter:
    hfr.add_host_fn0(bufwriter_new);
    hfr.add_host_fn3(bufwriter_write);
    hfr.add_host_fn1(bufwriter_commit);

    // Link:
    hfr.add_host_fn1(link_kind);
    hfr.add_host_fn2(link_eq);
    hfr.add_host_fn1(link_load_file);

    // BufReader:
    hfr.add_host_fn3(bufreader_read);

    log::debug!("Instantiated derive resolver: {:#?}", &hfr);
    hfr
}

fn bufwriter_new<S>(vm: &mut VirtualMachine<S>) -> Result<BufWriterHandle, Trap>
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

fn link_eq<S>(vm: &mut VirtualMachine<S>, a: LinkHandle<S>, b: LinkHandle<S>) -> Result<bool, Trap>
where
    S: Store,
{
    let linka = vm.links.get(a)?;
    let linkb = vm.links.get(b)?;
    Ok(linka == linkb)
}

fn link_load_file<S>(
    vm: &mut VirtualMachine<S>,
    handle: LinkHandle<S>,
) -> Result<BufReaderHandle, IOTrap>
where
    S: Store,
{
    let link = vm.links.get(handle)?;
    let fkey = link.get_file_key()?;
    let bytes = vm.nodestore.get_file(fkey)?;
    Ok(vm.brtab.insert(bytes))
}

fn bufreader_read<S>(
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
