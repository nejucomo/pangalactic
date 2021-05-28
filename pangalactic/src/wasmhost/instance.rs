use log::debug;

use super::externals::HostExternals;
use wasmi::{Error, RuntimeValue};

pub struct Instance {
    hext: HostExternals,
}

impl Instance {
    pub fn load_module(bytes: &[u8]) -> Result<Instance, Error> {
        use wasmi::{ImportsBuilder, Module, ModuleInstance};

        let module = Module::from_buffer(bytes)?;
        let mut hext = HostExternals::new();
        let imports = ImportsBuilder::new().with_resolver(env!("CARGO_PKG_NAME"), &hext);
        let modref = ModuleInstance::new(&module, &imports)?.assert_no_start();

        hext.register_mod(modref);

        debug!("Loaded module.");
        Ok(Instance { hext })
    }

    pub fn invoke_export(
        &mut self,
        name: &str,
        args: &[RuntimeValue],
    ) -> Result<Option<RuntimeValue>, Error> {
        debug!("invoke_export({:?}, {:?})", name, args);
        self.hext.invoke_export(name, args)
    }
}
