mod externals;
mod hostfuncs;

use self::hostfuncs::HostFuncs;
use wasmi::{Error, ModuleRef};

pub fn load_and_execute_module(bytes: &[u8]) -> Result<(), Error> {
    use log::debug;

    let funcs = HostFuncs::init();

    let modref = load_modref(&funcs, bytes)?;
    debug!("Loaded module.");

    let mut ext = self::externals::HostExternals::load(&modref, funcs)?;

    debug!("Executing: main()");
    let ret = modref.invoke_export("main", &[], &mut ext)?;
    debug!("Completed: main()");
    assert!(ret.is_none());

    Ok(())
}

fn load_modref(funcs: &HostFuncs, bytes: &[u8]) -> Result<ModuleRef, Error> {
    use wasmi::{ImportsBuilder, Module, ModuleInstance};

    let module = Module::from_buffer(bytes)?;
    let imports = ImportsBuilder::new().with_resolver(env!("CARGO_PKG_NAME"), funcs);
    let modref = ModuleInstance::new(&module, &imports)?.assert_no_start();
    Ok(modref)
}
