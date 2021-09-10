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

    let vm = VirtualMachine::load(store, exec)?;

    log::debug!("execute({:?})", input);
    let output = vm.execute(input)?;
    log::debug!("execute -> {:?}", &output);
    Ok(output)
}
