use crate::error::Result;
use pangalactic_node::Link;
use pangalactic_nodestore::NodeStore;
use pangalactic_store::Store;

pub fn derive<S>(
    store: &mut NodeStore<S>,
    exec: &Link<<S as Store>::Key>,
    input: &Link<<S as Store>::Key>,
) -> Result<Link<<S as Store>::Key>>
where
    S: Store,
{
    use crate::vm::VirtualMachine;

    log::debug!(
        "{}::derive{:?}",
        env!("CARGO_PKG_NAME"),
        (&store, &exec, &input)
    );

    let execkey = exec.get_file_key()?;
    let wasmbytes = store.get_file(execkey)?;
    log::trace!(
        "loading wasmbytes from {:?}, {} bytes",
        &execkey,
        wasmbytes.len()
    );

    let vm = VirtualMachine::load(wasmbytes)?;

    log::debug!("execute");
    vm.execute()?;
    todo!("Get output link.");
}
