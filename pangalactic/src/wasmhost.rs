mod externals;
mod hostfuncs;

use self::hostfuncs::HostFuncs;
use crate::error::Error;
use wasmi::ModuleRef;

pub async fn load_and_execute_module(bytes: &[u8]) -> Result<(), Error> {
    // Copy input for move across task boundary:
    let modbuf = Vec::from(bytes);

    tokio::task::spawn_blocking(move || {
        use log::debug;

        let funcs = HostFuncs::init();

        let modref = load_modref(&funcs, &modbuf[..])?;
        debug!("Loaded module.");

        use self::externals::HostExternals;

        let mut ext = HostExternals::load(&modref, funcs)?;

        debug!("Executing: pangalactic_main()");
        let ret = modref.invoke_export("pangalactic_main", &[], &mut ext)?;
        debug!("Completed: pangalactic_main()");
        assert!(ret.is_none());

        Ok(())
    })
    .await?
}

fn load_modref(funcs: &HostFuncs, bytes: &[u8]) -> Result<ModuleRef, Error> {
    use wasmi::{ImportsBuilder, Module, ModuleInstance};

    let module = Module::from_buffer(bytes)?;
    let imports = ImportsBuilder::new().with_resolver(env!("CARGO_PKG_NAME"), funcs);
    let modref = ModuleInstance::new(&module, &imports)?.assert_no_start();
    Ok(modref)
}
