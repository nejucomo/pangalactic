use crate::error::Result;
use pangalactic_nodestore::{LinkFor, NodeStore};
use pangalactic_store::Store;

pub fn derive<S>(
    store: &mut NodeStore<S>,
    exec: &LinkFor<S>,
    input: &LinkFor<S>,
) -> Result<LinkFor<S>>
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
