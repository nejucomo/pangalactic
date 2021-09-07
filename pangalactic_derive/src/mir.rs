/// mir - ModuleImportResolver for the dagwasm vm.
use wasmi::ValueType::I32;
use wasmi::{Error, FuncInstance, FuncRef, ModuleImportResolver, Signature, Trap, ValueType};

pub struct MIR(Vec<(&'static str, FuncRef)>);

impl MIR {
    pub fn new() -> MIR {
        let mut me = MIR(vec![]);

        me.register("log", &[I32, I32], None);

        me
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

impl ModuleImportResolver for MIR {
    fn resolve_func(&self, field_name: &str, signature: &Signature) -> Result<FuncRef, Error> {
        for (name, fi) in self.0.iter() {
            if field_name == *name && signature == fi.signature() {
                return Ok(fi.clone());
            }
        }
        return Err(Error::Instantiation(format!(
            "Export {} not found",
            field_name
        )));
    }
}
