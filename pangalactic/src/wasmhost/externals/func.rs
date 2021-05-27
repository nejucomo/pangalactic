use wasmi::{FuncRef, FuncInstance, RuntimeArgs, RuntimeValue, Trap, Signature, ValueType};


pub struct ExtFunc {
    name: &'static str,
    funcref: FuncRef,
}


impl ExtFunc {
    pub fn new(index: usize, name: &'static str, args: &'static [ValueType], ret: Option<ValueType>) -> ExtFunc {
        let sig = Signature::new(args, ret);
        let funcref = FuncInstance::alloc_host(sig, index);
        ExtFunc { name, funcref }
    }

    pub fn resolve(&self, reqname: &str, reqsig: &Signature) -> Result<Option<FuncRef>, String> {
        if reqname == self.name {
            if reqsig == self.funcref.signature() {
                Ok(Some(self.funcref.clone()))
            } else {
                Err(format!("Mismatched signature for host function {:?}", reqname))
            }
        } else {
            Ok(None)
        }
    }

    pub fn invoke(&self, args: RuntimeArgs) -> Result<Option<RuntimeValue>, Trap> {
        unimplemented!("ExtFunc<{:?}>.invoke({:?})", self.name, args)
    }
}
