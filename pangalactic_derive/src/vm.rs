mod hostfuncs;
mod table;

use self::table::{Handle, Table};
use crate::error::Result as DeriveResult;
use pangalactic_nodestore::{LinkFor, NodeStore};
use pangalactic_store::Store;
use pangalactic_wasmi::HostFuncResolver;
use wasmi::{Externals, MemoryRef, ModuleRef};

pub const WASM_ENTRYPOINT: &str = "derive";
pub const PANGALACTIC_BINDINGS: &str = "pangalactic_bindings";

pub struct VirtualMachine<'a, S>
where
    S: Store,
{
    hfr: HostFuncResolver<Self>,
    #[allow(dead_code)]
    nodestore: &'a mut NodeStore<S>,
    pub(crate) links: LinkTable<S>,
    exec: LinkHandle<S>,
    module: ModuleRef,
    #[allow(dead_code)]
    memory: MemoryRef,
}

pub type LinkTable<S> = Table<LinkFor<S>>;
pub type LinkHandle<S> = Handle<LinkFor<S>>;

pub type WasmiResult<T> = Result<T, wasmi::Error>;

impl<'a, S> VirtualMachine<'a, S>
where
    S: Store + 'static,
{
    pub fn load(nodestore: &'a mut NodeStore<S>, exec: &LinkFor<S>) -> DeriveResult<Self> {
        let hfr = self::hostfuncs::new_hostfunc_resolver::<S>();
        let wasmbytes = load_exec_wasm(nodestore, exec)?;
        let (module, memory) = init_mod::<S>(&hfr, exec, &wasmbytes)?;
        let mut links = Table::new();
        let exec = links.append(exec.clone());

        Ok(VirtualMachine {
            hfr,
            nodestore,
            links,
            exec,
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
        let outputlink = self.links.get(outputhandle)?.clone();
        Ok(outputlink)
    }
}

impl<'a, S> Externals for VirtualMachine<'a, S>
where
    S: Store,
{
    fn invoke_index(
        &mut self,
        index: usize,
        args: wasmi::RuntimeArgs<'_>,
    ) -> Result<Option<wasmi::RuntimeValue>, wasmi::Trap> {
        self.hfr.invoke_index(self, index, args)
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

fn init_mod<'a, S>(
    hfr: &HostFuncResolver<VirtualMachine<'a, S>>,
    exec: &LinkFor<S>,
    execbytes: &[u8],
) -> DeriveResult<(ModuleRef, MemoryRef)>
where
    S: Store,
{
    let module = load_modref(hfr, execbytes)?;
    log::debug!("Loaded module from {:?} ({} bytes)", exec, execbytes.len());
    let memory = resolve_memory(&module)?;
    log::trace!("Resolved memory.");
    Ok((module, memory))
}

fn load_modref<'a, S, B>(
    hfr: &HostFuncResolver<VirtualMachine<'a, S>>,
    bytes: B,
) -> WasmiResult<ModuleRef>
where
    B: AsRef<[u8]>,
    S: Store,
{
    use wasmi::{ImportsBuilder, Module, ModuleInstance};

    log::trace!(
        "Instantiating module from {} bytes...",
        bytes.as_ref().len()
    );
    let module = Module::from_buffer(bytes)?;
    let imports = ImportsBuilder::new().with_resolver(PANGALACTIC_BINDINGS, hfr);

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
