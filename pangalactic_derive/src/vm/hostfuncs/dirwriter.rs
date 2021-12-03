use super::iotrap::IOTrap;
use crate::vm::{DirWriterHandle, LinkHandle, VirtualMachine};
use pangalactic_node::Dir;
use pangalactic_store::Store;
use pangalactic_wasmi::Void;
use wasmi::Trap;

pub(super) fn dirwriter_new<S>(vm: &mut VirtualMachine<S>) -> Result<DirWriterHandle<S>, Trap>
where
    S: Store,
{
    Ok(vm.dwtab.insert(Dir::new()))
}

pub(super) fn dirwriter_add_link<S>(
    vm: &mut VirtualMachine<S>,
    dwh: DirWriterHandle<S>,
    nameptr: usize,
    namelen: usize,
    referent: LinkHandle<S>,
) -> Result<Void, IOTrap>
where
    S: Store,
{
    let dir = vm.dwtab.get_mut(dwh)?;
    let reflink = vm.links.remove(referent)?;
    vm.memory.with_direct_access(|mem| {
        let namebuf = &mem[nameptr..nameptr + namelen];
        let name = std::str::from_utf8(namebuf)?;
        dir.add_link(name, reflink);
        dbg!(&dir);
        Ok(Void)
    })
}

pub(super) fn dirwriter_commit<S>(
    vm: &mut VirtualMachine<S>,
    dwh: DirWriterHandle<S>,
) -> Result<LinkHandle<S>, IOTrap>
where
    S: Store,
{
    let dir = vm.dwtab.remove(dwh)?;
    let link = vm.nodestore.put_dir(&dir)?;
    let linkhandle = vm.links.insert(link);
    Ok(linkhandle)
}
