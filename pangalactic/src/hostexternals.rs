use wasmi::{
    Externals, RuntimeValue, RuntimeArgs, Error, ModuleImportResolver,
    FuncRef, ValueType, Signature, FuncInstance, Trap,
};

pub struct HostExternals {}

const INDEX_GET_BYTES: usize = 0;

impl Externals for HostExternals {
    fn invoke_index(
        &mut self,
        index: usize,
        args: RuntimeArgs,
    ) -> Result<Option<RuntimeValue>, Trap> {
        match index {
            INDEX_GET_BYTES => {
                let a: u32 = args.nth_checked(0)?;
                let b: u32 = args.nth_checked(1)?;
                println!("XXX GET_BYTES({:x}, {:x})", a, b);

                Ok(None)
            }
            _ => panic!("Unimplemented function at {}", index),
        }
    }
}

impl HostExternals {
    fn check_signature(
        &self,
        index: usize,
        signature: &Signature
    ) -> bool {
        let (params, ret_ty): (&[ValueType], Option<ValueType>) = match index {
            INDEX_GET_BYTES => (&[ValueType::I32, ValueType::I32], None),
            _ => return false,
        };
        signature.params() == params && signature.return_type() == ret_ty
    }
}

impl ModuleImportResolver for HostExternals {
    fn resolve_func(
        &self,
        field_name: &str,
        signature: &Signature
    ) -> Result<FuncRef, Error> {
        let index = match field_name {
            "get_bytes" => INDEX_GET_BYTES,
            _ => {
                return Err(Error::Instantiation(
                    format!("Export {} not found", field_name),
                ))
            }
        };

        if !self.check_signature(index, signature) {
            return Err(Error::Instantiation(
                format!("Export {} has a bad signature", field_name)
            ));
        }

        Ok(FuncInstance::alloc_host(
            Signature::new(&[ValueType::I32, ValueType::I32][..], None),
            index,
        ))
    }
}
