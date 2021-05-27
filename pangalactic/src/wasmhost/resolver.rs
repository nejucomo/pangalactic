use wasmi::{ImportResolver, Error, FuncRef, Signature, GlobalDescriptor, GlobalRef, MemoryDescriptor, MemoryRef, TableDescriptor, TableRef};

use super::externals::HostExternals;

pub struct Resolver {
    ext: HostExternals,
}

impl Resolver {
    pub fn new() -> Resolver {
        Resolver {
            ext: HostExternals::new(),
        }
    }
}

macro_rules! error {
    ( $fmt:expr, $( $arg:expr ),* ) => {
        Err(Error::Instantiation(format!($fmt, $( $arg ),* )))
    }
}

impl ImportResolver for Resolver {
    fn resolve_func(
        &self, 
        module_name: &str, 
        field_name: &str, 
        signature: &Signature
    ) -> Result<FuncRef, Error> {
        match module_name {
            env!("CARGO_PKG_NAME") =>
                self.ext.resolve_func(field_name, signature)
                .map_err(Error::Instantiation),
            _ =>
                error!("Unresolved WASM module {:?}", module_name)
        }
    }

    fn resolve_global(
        &self, 
        module_name: &str, 
        field_name: &str, 
        _descriptor: &GlobalDescriptor
    ) -> Result<GlobalRef, Error> {
        unimplemented!("resolve_global({:?}, {:?}, ...)", module_name, field_name);
    }

    fn resolve_memory(
        &self, 
        module_name: &str, 
        field_name: &str, 
        _descriptor: &MemoryDescriptor
    ) -> Result<MemoryRef, Error> {
        unimplemented!("resolve_memory({:?}, {:?}, ...)", module_name, field_name);
    }

    fn resolve_table(
        &self, 
        module_name: &str, 
        field_name: &str, 
        _descriptor: &TableDescriptor
    ) -> Result<TableRef, Error> {
        unimplemented!("resolve_tablet({:?}, {:?}, ...)", module_name, field_name);
    }
}


