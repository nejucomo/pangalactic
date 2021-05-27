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
        s.register_func(
            "phone_home", &[], None,
            Box::new(|args| {
                println!("host impl phone_home({:?})", args);
                None
            }),
        );
        s.register_func(
            "get_bytes", &[I32, I32], None,
            Box::new(|args| {
                println!("host impl get_bytes({:?})", args);
                unimplemented!("host impl get_bytes({:?})", args);
            }),
        );
        s
    }

    pub fn resolve_func(
        &self,
        field_name: &str,
        signature: &Signature
    ) -> Result<FuncRef, String> {
        println!("Externals::resolve_func({:?}, {:?})", field_name, signature);
        for extfunc in self.funcs.iter() {
            println!("... checking {:?}", extfunc);
            if let Some(funcref) = extfunc.resolve(field_name, signature)? {
                println!("    yep!");
                return Ok(funcref)
            }
            println!("    nope.");
        }
        println!("  Failed.");
        return Err(format!("No host function {:?} resolvable.", field_name))
    }

    fn register_func(&mut self, name: &'static str, args: &'static [ValueType], ret: Option<ValueType>, hostfunc: self::func::HostFuncBox) {
        let index = self.funcs.len();
        self.funcs.push(ExtFunc::new(index, name, args, ret, hostfunc));
    }
}


impl Externals for HostExternals {
    fn invoke_index(
        &mut self,
        index: usize,
        args: RuntimeArgs,
    ) -> Result<Option<RuntimeValue>, Trap> {
        use wasmi::TrapKind::TableAccessOutOfBounds;

        println!("HostExternals::invoke_index({:?}, {:?})", index, args);
        self
            .funcs
            .get_mut(index)
            .ok_or(TableAccessOutOfBounds)?
            .invoke(args)
    }
}
