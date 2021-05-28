mod func;

use log::{debug, info};
use self::func::ExtFunc;
use wasmi::{
    Error, Externals, FuncRef, GlobalDescriptor, GlobalRef, MemoryDescriptor, MemoryRef,
    ModuleImportResolver, RuntimeArgs, RuntimeValue, Signature, TableDescriptor, TableRef, Trap,
    ValueType, ModuleRef,
};

pub struct HostExternals {
    funcs: Vec<ExtFunc>,
    modref: Option<ModuleRef>,
}

impl HostExternals {
    pub fn new() -> HostExternals {
        use wasmi::ValueType::I32;

        let mut s = HostExternals { funcs: vec![], modref: None };
        s.register_func(
            "log",
            &[I32, I32],
            None,
            Box::new(|args| {
                let ptr: u32 = args.nth(0);
                let len: u32 = args.nth(1);
                let modref = s.modref.expect("modref not initialized");
                let memexp = modref.export_by_name("memory")
                    .expect("could not access guest 'memory' export");
                let memref = match memexp {
                    wasmi::ExternVal::Memory(memref) => memref,
                    other => panic!("Guest 'memory' export wrong type: {:?}", other),
                };
                let guestvec = memref.get(ptr, len as usize).unwrap();
                let guestbytes = &guestvec[..];
                match std::str::from_utf8(guestbytes) {
                    Ok(gueststr) => info!("[guest log] {}", gueststr),
                    Err(_) => info!("[guest log - malformed utf8] {:?}", guestbytes),
                };
                None
            }),
        );
        s
    }

    pub fn register_mod(&mut self, modref: ModuleRef) {
        assert!(self.modref.is_none());
        self.modref = Some(modref);
    }

    pub fn invoke_export(&mut self, name: &str, args: &[RuntimeValue]) -> Result<Option<RuntimeValue>, Error> {
        let modref = self.modref.as_mut()
            .expect("modref not initialized");
        modref.invoke_export(name, args, self)
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
            if let Some(funcref) = extfunc.resolve(field_name, signature)? {
                return Ok(funcref);
            }
        }
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

        self.funcs
            .get_mut(index)
            .ok_or(TableAccessOutOfBounds)?
            .invoke(args)
    }
}
