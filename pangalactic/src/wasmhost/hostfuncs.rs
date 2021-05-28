mod entry;

use self::entry::{CallResult, HFEntry};
use log::{debug, info};
use wasmi::{
    Error, FuncRef, GlobalDescriptor, GlobalRef, MemoryDescriptor, MemoryRef, ModuleImportResolver,
    RuntimeArgs, RuntimeValue, Signature, TableDescriptor, TableRef, Trap,
};

pub struct HostFuncs(Vec<HFEntry>);

impl HostFuncs {
    pub fn init() -> HostFuncs {
        use wasmi::ValueType::I32;

        let mut v = vec![];

        let mut regfunc = |name, args, ret, cb| {
            let ix = v.len();
            v.push(HFEntry::new(ix, name, args, ret, cb));
        };

        regfunc(
            "log",
            &[I32, I32],
            None,
            |mem: &MemoryRef, args: RuntimeArgs| -> CallResult {
                use wasmi::TrapKind::TableAccessOutOfBounds;

                let ptr: u32 = args.nth_checked(0)?;
                let len: u32 = args.nth_checked(1)?;
                let guestvec = mem
                    .get(ptr, len as usize)
                    .map_err(|_| TableAccessOutOfBounds)?;

                let guestbytes = &guestvec[..];
                match std::str::from_utf8(guestbytes) {
                    Ok(gueststr) => info!("[guest log] {}", gueststr),
                    Err(_) => info!("[guest log - malformed utf8] {:?}", guestbytes),
                };
                Ok(None)
            },
        );

        HostFuncs(v)
    }

    pub fn invoke_index(
        &self,
        mem: &MemoryRef,
        index: usize,
        args: RuntimeArgs,
    ) -> Result<Option<RuntimeValue>, Trap> {
        use wasmi::TrapKind::TableAccessOutOfBounds;

        let entry = self.0.get(index).ok_or(TableAccessOutOfBounds)?;
        entry.invoke(mem, args)
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

impl ModuleImportResolver for HostFuncs {
    fn resolve_func(&self, field_name: &str, signature: &Signature) -> Result<FuncRef, Error> {
        debug!(":resolve_func({:?}, {:?})", field_name, signature);
        for extfunc in self.0.iter() {
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
