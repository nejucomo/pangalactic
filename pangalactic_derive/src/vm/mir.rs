/// mir - ModuleImportResolver for the pangalactic vm.
use wasmi::ValueType::I64;
use wasmi::{Error, FuncInstance, FuncRef, ImportsBuilder, Signature, Trap, ValueType};

pub const PANGALACTIC_BINDINGS: &str = "pangalactic_bindings";

#[derive(Debug)]
pub struct ModuleImportResolver(Vec<(&'static str, FuncRef)>);

impl ModuleImportResolver {
    pub fn new() -> ModuleImportResolver {
        let mut me = ModuleImportResolver(vec![]);

        me.register("link_type", &[I64], Some(I64));

        log::trace!("Module Import Resolver {:#?}", &me);
        me
    }

    pub fn make_imports_builder(&self) -> ImportsBuilder {
        log::trace!("ImportsBuilder for {:?}", PANGALACTIC_BINDINGS);
        ImportsBuilder::new().with_resolver(PANGALACTIC_BINDINGS, self)
    }

    fn register(
        &mut self,
        name: &'static str,
        argtypes: &'static [ValueType],
        ret: Option<ValueType>,
    ) {
        let idx = self.0.len();
        let fi = FuncInstance::alloc_host(Signature::new(argtypes, ret), idx);
        self.0.push((name, fi));
    }

    pub fn get_index(&self, idx: usize) -> Result<&FuncRef, Trap> {
        use wasmi::TrapKind::TableAccessOutOfBounds;
        let (_, fr) = self.0.get(idx).ok_or(Trap::new(TableAccessOutOfBounds))?;
        Ok(fr)
    }
}

impl wasmi::ModuleImportResolver for ModuleImportResolver {
    fn resolve_func(&self, field_name: &str, signature: &Signature) -> Result<FuncRef, Error> {
        for (name, fi) in self.0.iter() {
            if field_name == *name && signature == fi.signature() {
                return Ok(fi.clone());
            }
        }
        return Err(Error::Instantiation(format!(
            "Export {} {:?} not found",
            field_name, signature,
        )));
    }
}
