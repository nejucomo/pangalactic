use wasmi::{FuncRef, FuncInstance, RuntimeArgs, RuntimeValue, Trap, Signature, ValueType};


pub type HostFuncBox = Box<dyn FnMut(RuntimeArgs) -> Option<RuntimeValue>>;
pub struct ExtFunc {
    name: &'static str,
    funcref: FuncRef,
    hostfunc: HostFuncBox,
}


impl ExtFunc {
    pub fn new(index: usize, name: &'static str, args: &'static [ValueType], ret: Option<ValueType>, hostfunc: HostFuncBox) -> ExtFunc
    {
        let sig = Signature::new(args, ret);
        let funcref = FuncInstance::alloc_host(sig, index);
        ExtFunc { name, funcref, hostfunc }
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
        Ok(self.hostfunc.call_mut((args,)))
    }
}
