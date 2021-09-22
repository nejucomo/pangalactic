use crate::{HostFunc, HostFuncAdapter};
use wasmi::{
    Error, Externals, FuncRef, ModuleImportResolver, RuntimeArgs, RuntimeValue, Signature, Trap,
};

pub struct HostFuncResolver(Vec<Entry>);

struct Entry {
    hf: Box<dyn HostFuncAdapter>,
    funcref: FuncRef,
}

impl HostFuncResolver {
    pub fn new() -> HostFuncResolver {
        HostFuncResolver(vec![])
    }

    pub fn add_host_func<H>(&mut self, hostfunc: H)
    where
        H: HostFunc + 'static,
    {
        // BUG? Why is 'static necessary since hostfunc moves into Entry?
        use wasmi::FuncInstance;

        let funcref = FuncInstance::alloc_host(hostfunc.signature(), self.0.len());
        let hf = Box::new(hostfunc);

        self.0.push(Entry { hf, funcref });
    }
}

impl ModuleImportResolver for HostFuncResolver {
    fn resolve_func(&self, field_name: &str, signature: &Signature) -> Result<FuncRef, Error> {
        for entry in self.0.iter() {
            if entry.hf.name() == field_name && entry.funcref.signature() == signature {
                return Ok(entry.funcref.clone());
            }
        }

        return Err(Error::Instantiation(format!(
            "Export {} not found",
            field_name
        )));
    }
}

impl Externals for HostFuncResolver {
    fn invoke_index(
        &mut self,
        index: usize,
        args: RuntimeArgs<'_>,
    ) -> Result<Option<RuntimeValue>, Trap> {
        use wasmi::TrapKind::TableAccessOutOfBounds;

        let entry = self.0.get(index).ok_or(Trap::new(TableAccessOutOfBounds))?;
        entry.hf.invoke(args)
    }
}
