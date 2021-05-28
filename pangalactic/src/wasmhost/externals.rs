use self::func::ExtFunc;
use super::hostfuncs::HostFuncs;
use log::{debug, info};
use wasmi::{
    Error, Externals, FuncRef, GlobalDescriptor, GlobalRef, MemoryDescriptor, MemoryRef,
    ModuleImportResolver, ModuleRef, RuntimeArgs, RuntimeValue, Signature, TableDescriptor,
    TableRef, Trap, ValueType,
};

pub struct HostExternals {
    mem: MemoryRef,
    funcs: HostFuncs,
}

impl HostExternals {
    pub fn load(modref: &ModuleRef, funcs: HostFuncs) -> Result<HostExternals, Error> {
        use wasmi::ExternVal::Memory;

        let export = modref
            .export_by_name("memory")
            .ok_or(Error::Instantiation("Could not find 'memory' export.'"))?;
        let mem = match export {
            Memory(m) => Ok(m),
            other => Err(Error::Instantiation(format!(
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
        use wasmi::TrapKind::TableAccessOutOfBounds;

        self.funcs.invoke_index(&self.memref, index, args)
    }
}
