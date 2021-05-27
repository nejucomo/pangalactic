use wasmi::{Error, ModuleRef, RuntimeValue};


pub struct Instance {
    #[allow(dead_code)]
    modref: ModuleRef,
}


impl Instance {
    pub fn load_module(bytes: &[u8]) -> Result<Instance, Error> {
        use wasmi::{Module, ModuleInstance};
        use super::resolver::Resolver;

        let module = Module::from_buffer(bytes)?;
        let modref =
            ModuleInstance::new(
                &module,
                &Resolver::new(),
            )?
            .assert_no_start();

        Ok(Instance { modref })
    }

    pub fn invoke_export(&self, name: &str,  args: &[RuntimeValue]) -> Result<Option<RuntimeValue>, Error> {
        unimplemented!("invoke_export({:?}, {:?})", name, args);
    }
}
