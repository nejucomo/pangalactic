use wasmi::{ImportResolver, Error, FuncRef, Signature, GlobalDescriptor, GlobalRef, MemoryDescriptor, MemoryRef, TableDescriptor, TableRef};

pub struct Resolver {}

impl Resolver {
    pub fn new() -> Resolver {
        Resolver {}
    }
}

impl ImportResolver for Resolver {
    fn resolve_func(
        &self, 
        module_name: &str, 
        field_name: &str, 
        signature: &Signature
    ) -> Result<FuncRef, Error> {
        unimplemented!("resolve_func({:?}, {:?}, {:#?})", module_name, field_name, signature);
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


