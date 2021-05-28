use super::hostfuncs::HostFuncs;
use wasmi::{Error, Externals, MemoryRef, ModuleRef, RuntimeArgs, RuntimeValue, Trap};

pub struct HostExternals {
    mem: MemoryRef,
    funcs: HostFuncs,
}

impl HostExternals {
    pub fn load(modref: &ModuleRef, funcs: HostFuncs) -> Result<HostExternals, Error> {
        use wasmi::Error::Instantiation;
        use wasmi::ExternVal::Memory;

        let export = modref
            .export_by_name("memory")
            .ok_or(Instantiation(format!("Could not find 'memory' export.'")))?;
        let mem = match export {
            Memory(m) => Ok(m),
            other => Err(Instantiation(format!(
                "Invalid 'memory' export type: {:?}",
                other
            ))),
        }?;

        Ok(HostExternals { mem, funcs })
    }
}

impl Externals for HostExternals {
    fn invoke_index(
        &mut self,
        index: usize,
        args: RuntimeArgs,
    ) -> Result<Option<RuntimeValue>, Trap> {
        self.funcs.invoke_index(&self.mem, index, args)
    }
}
