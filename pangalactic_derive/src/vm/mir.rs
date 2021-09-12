use crate::vm::hostfunc::HostFunc;
use crate::vm::VirtualMachine;
use pangalactic_store::Store;
use wasmi::{
    Error, FuncInstance, FuncRef, ImportsBuilder, RuntimeArgs, RuntimeValue, Signature, Trap,
};

pub const PANGALACTIC_BINDINGS: &str = "pangalactic_bindings";

#[derive(Debug)]
pub struct ModuleImportResolver(Vec<Entry>);

#[derive(Debug)]
struct Entry {
    name: &'static str,
    funcref: FuncRef,
}

impl ModuleImportResolver {
    pub fn new() -> ModuleImportResolver {
        use enum_iterator::IntoEnumIterator;

        let mut hfs = vec![];

        for (idx, hf) in HostFunc::into_enum_iter().enumerate() {
            hfs.push(Entry {
                name: hf.name(),
                funcref: FuncInstance::alloc_host(hf.signature(), idx),
            });
        }

        ModuleImportResolver(hfs)
    }

    pub fn make_imports_builder(&self) -> ImportsBuilder {
        log::trace!("ImportsBuilder for {:?}", PANGALACTIC_BINDINGS);
        ImportsBuilder::new().with_resolver(PANGALACTIC_BINDINGS, self)
    }

    pub fn invoke_index<'a, S>(
        vm: &mut VirtualMachine<'a, S>,
        index: usize,
        args: RuntimeArgs<'_>,
    ) -> Result<Option<RuntimeValue>, Trap>
    where
        S: Store,
    {
        use num_traits::cast::FromPrimitive;
        use wasmi::TrapKind::TableAccessOutOfBounds;

        let hf = HostFunc::from_usize(index).ok_or(Trap::new(TableAccessOutOfBounds))?;

        hf.invoke(vm, args)
    }
}

impl wasmi::ModuleImportResolver for ModuleImportResolver {
    fn resolve_func(&self, field_name: &str, signature: &Signature) -> Result<FuncRef, Error> {
        for entry in self.0.iter() {
            if field_name == entry.name && signature == entry.funcref.signature() {
                return Ok(entry.funcref.clone());
            }
        }
        return Err(Error::Instantiation(format!(
            "Export {} {:?} not found",
            field_name, signature,
        )));
    }
}
