mod mir;

use self::mir::ModuleImportResolver;
use pangalactic_nodestore::{LinkFor, NodeStore};
use pangalactic_store::Store;
use wasmi::{Error, MemoryRef, ModuleRef};

pub struct VirtualMachine<'a, S>
where
    S: Store,
{
    nodestore: &'a mut NodeStore<S>,
    exec: LinkFor<S>,
    mir: ModuleImportResolver,
    module: ModuleRef,

    #[allow(dead_code)]
    memory: MemoryRef,
}

impl<'a, S> VirtualMachine<'a, S>
where
    S: Store,
{
    pub fn load(nodestore: &'a mut NodeStore<S>, exec: &LinkFor<S>) -> crate::error::Result<Self> {
        let exec = exec.clone();
        let execkey = exec.get_file_key()?;
        let wasmbytes = nodestore.get_file(execkey)?;
        let mir = ModuleImportResolver::new();
        let module = load_modref(&mir, &wasmbytes)?;
        log::debug!("Loaded module from {:?} ({} bytes)", exec, wasmbytes.len());
        let memory = resolve_memory(&module)?;
        log::trace!("Resolved memory.");

        Ok(VirtualMachine {
            nodestore,
            exec,
            mir,
            module,
            memory,
        })
    }

    pub fn execute(mut self, input: &LinkFor<S>) -> Result<LinkFor<S>, Error> {
        log::debug!("Executing: VirtualMachine::execute()");
        let modref = self.module.clone();
        let ret =
            modref.invoke_export("pangalactic_derive", &[self.exec, input.clone()], &mut self)?;
        log::debug!("Completed: VirtualMachine::execute()");

        let retval = ret.ok_or({
            use wasmi::{Trap, TrapKind::UnexpectedSignature};
            Trap::new(UnexpectedSignature)
        })?;

        Ok(retval)
    }
}

impl<'a, S> wasmi::Externals for VirtualMachine<'a, S>
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
