use wasmi::{Error, FuncRef, MemoryRef, RuntimeArgs, RuntimeValue, Signature, Trap, ValueType};

pub type CallResult = Result<Option<RuntimeValue>, Trap>;

pub struct HFEntry {
    name: &'static str,
    funcref: FuncRef,
    hostfunc: HostFuncBox,
}

type HostFuncBox = Box<dyn Fn(&MemoryRef, RuntimeArgs) -> CallResult>;

impl HFEntry {
    pub fn new<F>(
        index: usize,
        name: &'static str,
        args: &'static [ValueType],
        ret: Option<ValueType>,
        hostfunc: F,
    ) -> HFEntry
    where
        F: 'static + Fn(&MemoryRef, RuntimeArgs) -> CallResult,
    {
        use wasmi::FuncInstance;

        let sig = Signature::new(args, ret);
        let funcref = FuncInstance::alloc_host(sig, index);
        HFEntry {
            name,
            funcref,
            hostfunc: Box::new(hostfunc),
        }
    }

    pub fn resolve(&self, reqname: &str, reqsig: &Signature) -> Result<Option<FuncRef>, Error> {
        if reqname == self.name {
            if reqsig == self.funcref.signature() {
                Ok(Some(self.funcref.clone()))
            } else {
                Err(Error::Function(format!(
                    "Mismatched signature for host function {:?}",
                    reqname
                )))
            }
        } else {
            Ok(None)
        }
    }

    pub fn invoke(&self, mem: &MemoryRef, args: RuntimeArgs) -> CallResult {
        (self.hostfunc)(mem, args)
    }
}
