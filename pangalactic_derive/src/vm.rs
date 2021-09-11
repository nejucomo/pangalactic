mod mir;
mod table;

use self::mir::ModuleImportResolver;
use self::table::{Handle, Table};
use crate::error::Result as DeriveResult;
use pangalactic_nodestore::{LinkFor, NodeStore};
use pangalactic_store::Store;
use wasmi::{MemoryRef, ModuleRef};

pub const WASM_ENTRYPOINT: &str = "pangalactic_derive";

pub struct VirtualMachine<'a, S>
where
    S: Store,
{
    #[allow(dead_code)]
    nodestore: &'a mut NodeStore<S>,
    links: LinkTable<S>,
    exec: LinkHandle<S>,
    mir: ModuleImportResolver,
    module: ModuleRef,
    #[allow(dead_code)]
    memory: MemoryRef,
}

pub type LinkTable<S> = Table<LinkFor<S>>;
pub type LinkHandle<S> = Handle<LinkFor<S>>;

pub type WasmiResult<T> = Result<T, wasmi::Error>;

impl<'a, S> VirtualMachine<'a, S>
where
    S: Store,
{
    pub fn load(nodestore: &'a mut NodeStore<S>, exec: &LinkFor<S>) -> DeriveResult<Self> {
        let wasmbytes = load_exec_wasm(nodestore, exec)?;
        let (mir, module, memory) = init_mod::<S>(exec, &wasmbytes)?;
        let mut links = Table::new();
        let exec = links.append(exec.clone());

        Ok(VirtualMachine {
            nodestore,
            links,
            exec,
            mir,
            module,
            memory,
        })
    }

    pub fn execute(mut self, input: &LinkFor<S>) -> WasmiResult<LinkFor<S>> {
        use std::convert::TryFrom;
        use wasmi::RuntimeValue;

        log::debug!("Executing: VirtualMachine::execute()");
        let inputhandle = self.links.append(input.clone());
        let args = &[
            RuntimeValue::try_from(self.exec)?,
            RuntimeValue::try_from(inputhandle)?,
        ];
        let modref = self.module.clone();
        let ret = modref.invoke_export(WASM_ENTRYPOINT, args, &mut self)?;
        log::debug!("Completed: VirtualMachine::execute()");

        let outputval = ret.ok_or({
            use wasmi::{Trap, TrapKind::UnexpectedSignature};
            Trap::new(UnexpectedSignature)
        })?;

        let outputhandle = LinkHandle::<S>::try_from(outputval)?;
        let outputlink = self.links[outputhandle].clone();
        Ok(outputlink)
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
fn load_exec_wasm<S>(ns: &mut NodeStore<S>, exec: &LinkFor<S>) -> DeriveResult<Vec<u8>>
where
    S: Store,
{
    let execkey = exec.get_file_key()?;
    let wasmbytes = ns.get_file(execkey)?;
    Ok(wasmbytes)
}

fn init_mod<S>(
    exec: &LinkFor<S>,
    execbytes: &[u8],
) -> DeriveResult<(ModuleImportResolver, ModuleRef, MemoryRef)>
where
    S: Store,
{
    let mir = ModuleImportResolver::new();
    let module = load_modref(&mir, execbytes)?;
    log::debug!("Loaded module from {:?} ({} bytes)", exec, execbytes.len());
    let memory = resolve_memory(&module)?;
    log::trace!("Resolved memory.");
    Ok((mir, module, memory))
}

fn load_modref<B>(mir: &ModuleImportResolver, bytes: B) -> WasmiResult<ModuleRef>
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

fn resolve_memory(modref: &ModuleRef) -> WasmiResult<MemoryRef> {
    use wasmi::Error::Instantiation;
    use wasmi::ExternVal::Memory;

    let export = modref
        .export_by_name("memory")
        .ok_or(Instantiation(format!("Could not find 'memory' export.")))?;

    match export {
        Memory(m) => Ok(m),
        other => Err(Instantiation(format!(
            "Invalid 'memory' export type: {:?}",
            other
        ))),
    }
}
