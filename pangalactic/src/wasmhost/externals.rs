mod func;

use log::debug;
use self::func::ExtFunc;
use wasmi::{
    Error, Externals, FuncRef, GlobalDescriptor, GlobalRef, MemoryDescriptor, MemoryRef,
    ModuleImportResolver, RuntimeArgs, RuntimeValue, Signature, TableDescriptor, TableRef, Trap,
    ValueType,
};

pub struct HostExternals {
    funcs: Vec<ExtFunc>,
}

impl HostExternals {
    pub fn new() -> HostExternals {
        use wasmi::ValueType::I32;

        let mut s = HostExternals { funcs: vec![] };
        s.register_func(
            "log",
            &[I32, I32],
            None,
            Box::new(|args| {
                debug!("host impl log({:?})", args);
                let ptr: u32 = args.nth(0);
                let len: u32 = args.nth(1);
                unimplemented!("host impl log({:?}, {:?})", ptr, len);
            }),
        );
        s.register_func(
            "phone_home",
            &[],
            None,
            Box::new(|args| {
                debug!("host impl phone_home({:?})", args);
                None
            }),
        );
        s.register_func(
            "get_bytes",
            &[I32, I32],
            None,
            Box::new(|args| {
                debug!("host impl get_bytes({:?})", args);
                unimplemented!("host impl get_bytes({:?})", args);
            }),
        );
        s
    }

    fn register_func(
        &mut self,
        name: &'static str,
        args: &'static [ValueType],
        ret: Option<ValueType>,
        hostfunc: self::func::HostFuncBox,
    ) {
        let index = self.funcs.len();
        self.funcs
            .push(ExtFunc::new(index, name, args, ret, hostfunc));
    }
}

macro_rules! not_found {
    ( $errCtr:ident, $name:expr ) => {{
        use wasmi::Error::$errCtr;

        Err($errCtr(format!(
            "Host {} not found: {:?}",
            stringify!($errCtr),
            $name
        )))
    }};
}

impl ModuleImportResolver for HostExternals {
    fn resolve_func(&self, field_name: &str, signature: &Signature) -> Result<FuncRef, Error> {
        debug!("Externals::resolve_func({:?}, {:?})", field_name, signature);
        for extfunc in self.funcs.iter() {
            debug!("... checking {:?}", extfunc);
            if let Some(funcref) = extfunc.resolve(field_name, signature)? {
                debug!("    yep!");
                return Ok(funcref);
            }
            debug!("    nope.");
        }
        debug!("  Failed.");
        return not_found!(Function, field_name);
    }

    fn resolve_global(
        &self,
        field_name: &str,
        _global_type: &GlobalDescriptor,
    ) -> Result<GlobalRef, Error> {
        not_found!(Global, field_name)
    }

    fn resolve_memory(
        &self,
        field_name: &str,
        _memory_type: &MemoryDescriptor,
    ) -> Result<MemoryRef, Error> {
        not_found!(Memory, field_name)
    }

    fn resolve_table(
        &self,
        field_name: &str,
        _table_type: &TableDescriptor,
    ) -> Result<TableRef, Error> {
        not_found!(Table, field_name)
    }
}

impl Externals for HostExternals {
    fn invoke_index(
        &mut self,
        index: usize,
        args: RuntimeArgs,
    ) -> Result<Option<RuntimeValue>, Trap> {
        use wasmi::TrapKind::TableAccessOutOfBounds;

        debug!("HostExternals::invoke_index({:?}, {:?})", index, args);
        self.funcs
            .get_mut(index)
            .ok_or(TableAccessOutOfBounds)?
            .invoke(args)
    }
}
