mod func;

use wasmi::{
    Externals, RuntimeValue, RuntimeArgs,
    FuncRef, ValueType, Signature, Trap,
};
use self::func::ExtFunc;


pub struct HostExternals {
    funcs: Vec<ExtFunc>,
}


impl HostExternals {
    pub fn new() -> HostExternals {
        use wasmi::ValueType::I32;

        let mut s = HostExternals { funcs: vec![] };
        s.register_func("get_bytes", &[I32, I32], None);
        s
    }

    pub fn resolve_func(
        &self, 
        field_name: &str, 
        signature: &Signature
    ) -> Result<FuncRef, String> {
        for extfunc in self.funcs.iter() {
            if let Some(funcref) = extfunc.resolve(field_name, signature)? {
                return Ok(funcref)
            }
        }
        return Err(format!("No host function {:?} resolvable.", field_name))
    }

    fn register_func(&mut self, name: &'static str, args: &'static [ValueType], ret: Option<ValueType>) {
        let index = self.funcs.len();
        self.funcs.push(ExtFunc::new(index, name, args, ret));
    }
}


impl Externals for HostExternals {
    fn invoke_index(
        &mut self,
        index: usize,
        args: RuntimeArgs,
    ) -> Result<Option<RuntimeValue>, Trap> {
        use wasmi::TrapKind::TableAccessOutOfBounds;

        self
            .funcs
            .get(index)
            .ok_or(TableAccessOutOfBounds)?
            .invoke(args)
    }
}
