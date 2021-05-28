use log::debug;

use super::externals::HostExternals;
use wasmi::{Error, ModuleRef, RuntimeValue};

pub struct Instance {
    hext: HostExternals,
    modref: ModuleRef,
}

impl Instance {
    pub fn load_module(bytes: &[u8]) -> Result<Instance, Error> {
        use wasmi::{ImportsBuilder, Module, ModuleInstance};

        let module = Module::from_buffer(bytes)?;
        let hext = HostExternals::new();
        let imports = ImportsBuilder::new().with_resolver(env!("CARGO_PKG_NAME"), &hext);
        let modref = ModuleInstance::new(&module, &imports)?.assert_no_start();

        debug!("Loaded module.");
        Ok(Instance { hext, modref })
    }

    pub fn invoke_export(
        &mut self,
        name: &str,
        args: &[RuntimeValue],
    ) -> Result<Option<RuntimeValue>, Error> {
        debug!("invoke_export({:?}, {:?})", name, args);
        self.modref.invoke_export(name, args, &mut self.hext)
    }
}
