use crate::mir::MIR;
use pangalactic_dagnode::DagNode;
use pangalactic_store::Store;
use wasmi::ModuleRef;

pub struct VirtualMachine<S>
where
    S: Store,
{
    store: S,
    writers: Vec<Option<DagNode<<S as Store>::Key>>>,
    readers: Vec<Option<DagNode<<S as Store>::Key>>>,
    links: Vec<<S as Store>::Key>,
    mir: MIR,
}

impl<S> VirtualMachine<S>
where
    S: Store,
{
    pub fn new(store: S) -> VirtualMachine<S> {
        VirtualMachine {
            store,
            writers: vec![],
            readers: vec![],
            links: vec![],
            mir: MIR::new(),
        }
    }

    pub fn load_modref(&self, bytes: &[u8]) -> Result<ModuleRef, wasmi::Error> {
        use wasmi::{ImportsBuilder, Module, ModuleInstance};

        let module = Module::from_buffer(bytes)?;
        let imports = ImportsBuilder::new().with_resolver(env!("CARGO_PKG_NAME"), &self.mir);
        let modref = ModuleInstance::new(&module, &imports)?.assert_no_start();
        Ok(modref)
    }
}

impl<S> wasmi::Externals for VirtualMachine<S>
where
    S: Store,
{
    fn invoke_index(
        &mut self,
        index: usize,
        args: wasmi::RuntimeArgs<'_>,
    ) -> Result<Option<wasmi::RuntimeValue>, wasmi::Trap> {
        use wasmi::FuncInstance;

        let func = self.mir.get_index(index)?.clone();
        FuncInstance::invoke(&func, args.as_ref(), self)
    }
}
