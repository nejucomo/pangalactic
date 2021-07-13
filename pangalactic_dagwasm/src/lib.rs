mod mir;
mod vm;

use pangalactic_store::Store;

pub fn load_and_execute_module<S>(store: S, bytes: &[u8]) -> Result<(), wasmi::Error>
where
    S: Store,
{
    let vm = vm::VirtualMachine::load(store, bytes)?;
    vm.execute()
}
