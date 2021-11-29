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
    hfr.add_host_fn1(link_kind);
    hfr.add_host_fn1(load_file);
    log::debug!("Instantiated derive resolver: {:#?}", &hfr);
    hfr
}

fn new_file<S>(vm: &mut VirtualMachine<S>) -> Result<BufWriterHandle, Trap>
where
    S: Store,
{
    Ok(vm.bwtab.append(vec![]))
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

fn link_kind<S>(vm: &mut VirtualMachine<S>, handle: LinkHandle<S>) -> Result<Kind, Trap>
where
    S: Store,
{
    let link = vm.links.get(handle)?;
    Ok(link.kind)
}

#[derive(derive_more::From)]
enum ReadFileError {
    Trap(Trap),
    Stdio(std::io::Error),
}

impl From<ReadFileError> for Trap {
    fn from(rfe: ReadFileError) -> Trap {
        match rfe {
            ReadFileError::Trap(t) => t,
            ReadFileError::Stdio(e) => pangalactic_wasmi::into_trap(e),
        }
    }
}

fn load_file<S>(
    vm: &mut VirtualMachine<S>,
    handle: LinkHandle<S>,
) -> Result<ReadHandle, ReadFileError>
where
    S: Store,
{
    let link = vm.links.get(handle)?;
    let fkey = link.get_file_key()?;
    let bytes = vm.nodestore.get_file(fkey)?;
    Ok(vm.readtab.append(bytes))
}
