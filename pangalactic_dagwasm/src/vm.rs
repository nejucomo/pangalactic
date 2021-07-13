use crate::mir::MIR;
use pangalactic_dagnode::DagNode;
use pangalactic_store::Store;
use wasmi::{Error, MemoryRef, ModuleRef};

pub struct VirtualMachine<S>
where
    S: Store,
{
    store: S,
    writers: Vec<Option<DagNode<<S as Store>::Key>>>,
    readers: Vec<Option<DagNode<<S as Store>::Key>>>,
    links: Vec<<S as Store>::Key>,
    mir: MIR,
    module: ModuleRef,
    memory: MemoryRef,
}

impl<S> VirtualMachine<S>
where
    S: Store,
{
    pub fn load(store: S, modbytes: &[u8]) -> Result<VirtualMachine<S>, Error> {
        use log::debug;

        let mir = MIR::new();
        let module = load_modref(&mir, modbytes)?;
        debug!("Loaded module.");
        let memory = resolve_memory(&module)?;

        Ok(VirtualMachine {
            store,
            writers: vec![],
            readers: vec![],
            links: vec![],
            mir,
            module,
            memory,
        })
    }

    pub fn execute(mut self) -> Result<(), Error> {
        log::debug!("Executing: VirtualMachine::execute()");
        let modref = self.module.clone();
        let ret = modref.invoke_export("pangalactic_main", &[], &mut self)?;
        log::debug!("Completed: VirtualMachine::execute()");

        if let Some(noise) = ret {
            use wasmi::{Trap, TrapKind};
            log::warn!("Unexpected return value: {:?}", noise);
            Err(Error::Trap(Trap::new(TrapKind::UnexpectedSignature)))
        } else {
            Ok(())
        }
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

// Private helper funcs:
fn load_modref(mir: &MIR, bytes: &[u8]) -> Result<ModuleRef, Error> {
    use wasmi::{ImportsBuilder, Module, ModuleInstance};

    let module = Module::from_buffer(bytes)?;
    let imports = ImportsBuilder::new().with_resolver(env!("CARGO_PKG_NAME"), mir);
    let modref = ModuleInstance::new(&module, &imports)?.assert_no_start();
    Ok(modref)
}

fn resolve_memory(modref: &ModuleRef) -> Result<MemoryRef, Error> {
    use wasmi::ExternVal::Memory;

    let export = modref
        .export_by_name("memory")
        .ok_or(Error::Instantiation(format!(
            "Could not find 'memory' export.'"
        )))?;

    match export {
        Memory(m) => Ok(m),
        other => Err(Error::Instantiation(format!(
            "Invalid 'memory' export type: {:?}",
            other
        ))),
    }
}
