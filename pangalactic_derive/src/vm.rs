mod mir;

use self::mir::ModuleImportResolver;
use wasmi::{Error, MemoryRef, ModuleRef};

pub struct VirtualMachine {
    mir: ModuleImportResolver,
    module: ModuleRef,

    #[allow(dead_code)]
    memory: MemoryRef,
}

impl VirtualMachine {
    pub fn load<B>(modbytes: B) -> Result<VirtualMachine, Error>
    where
        B: AsRef<[u8]>,
    {
        let mir = ModuleImportResolver::new();
        let module = load_modref(&mir, modbytes)?;
        log::debug!("Loaded module.");
        let memory = resolve_memory(&module)?;
        log::trace!("Resolved memory.");

        Ok(VirtualMachine {
            mir,
            module,
            memory,
        })
    }

    pub fn execute(mut self) -> Result<(), Error> {
        log::debug!("Executing: VirtualMachine::execute()");
        let modref = self.module.clone();
        let ret = modref.invoke_export("pangalactic_derive", &[], &mut self)?;
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

impl wasmi::Externals for VirtualMachine {
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
fn load_modref<B>(mir: &ModuleImportResolver, bytes: B) -> Result<ModuleRef, Error>
where
    B: AsRef<[u8]>,
{
    use wasmi::{ImportsBuilder, Module, ModuleInstance};

    log::trace!(
        "Instantiating module from {} bytes...",
        bytes.as_ref().len()
    );
    let module = Module::from_buffer(bytes)?;

    log::trace!("Resolving imports for {}...", env!("CARGO_PKG_NAME"));
    let imports = ImportsBuilder::new().with_resolver(env!("CARGO_PKG_NAME"), mir);

    log::trace!("Instantiating Module...");
    let modref = ModuleInstance::new(&module, &imports)?.assert_no_start();
    Ok(modref)
}

fn resolve_memory(modref: &ModuleRef) -> Result<MemoryRef, Error> {
    use wasmi::ExternVal::Memory;

    let export = modref
        .export_by_name("memory")
        .ok_or(Error::Instantiation(format!(
            "Could not find 'memory' export."
        )))?;

    match export {
        Memory(m) => Ok(m),
        other => Err(Error::Instantiation(format!(
            "Invalid 'memory' export type: {:?}",
            other
        ))),
    }
}
