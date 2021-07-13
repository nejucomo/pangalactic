mod mir;
mod vm;

use pangalactic_store::Store;

pub fn load_and_execute_module<S>(store: S, bytes: &[u8]) -> Result<(), wasmi::Error>
where
    S: Store,
{
    use log::debug;

    let vm = vm::VirtualMachine::new(store);
    let modref = vm.load_modref(bytes)?;
    debug!("Loaded module.");

    todo!();

    /*
    let mut ext = HostExternals::load(&modref, funcs)?;

    debug!("Executing: pangalactic_main()");
    let ret = modref.invoke_export("pangalactic_main", &[], &mut ext)?;
    debug!("Completed: pangalactic_main()");
    assert!(ret.is_none());
    */
}
