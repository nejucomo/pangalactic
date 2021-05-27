use wasmi::{Error, ModuleRef, RuntimeValue};
use super::resolver::Resolver;


pub struct Instance {
    res: Resolver,
    modref: ModuleRef,
}


impl Instance {
    pub fn load_module(bytes: &[u8]) -> Result<Instance, Error> {
        use wasmi::{Module, ModuleInstance};

        let module = Module::from_buffer(bytes)?;
        let res = Resolver::new();
        let modref =
            ModuleInstance::new(
                &module,
                &res,
            )?
            .assert_no_start();

        println!("Loaded module...");
        Ok(Instance { res, modref })
    }

    pub fn invoke_export(&mut self, name: &str,  args: &[RuntimeValue]) -> Result<Option<RuntimeValue>, Error> {
        println!("Instance::invoke_export({:?}, {:?})", name, args);
        self.res.invoke_export(self.modref.clone(), name, args)
    }
}
