use wasmi::{Error, FuncInstance, FuncRef, RuntimeArgs, RuntimeValue, Signature, Trap, ValueType};

pub type HostFuncBox = Box<dyn FnMut(RuntimeArgs) -> Option<RuntimeValue>>;

pub struct ExtFunc {
    name: &'static str,
    funcref: FuncRef,
    hostfunc: HostFuncBox,
}

impl std::fmt::Debug for ExtFunc {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "ExtFunc<{}>", self.name)
    }
}

impl ExtFunc {
    pub fn new(
        index: usize,
        name: &'static str,
        args: &'static [ValueType],
        ret: Option<ValueType>,
        hostfunc: HostFuncBox,
    ) -> ExtFunc {
        let sig = Signature::new(args, ret);
        let funcref = FuncInstance::alloc_host(sig, index);
        ExtFunc {
            name,
            funcref,
            hostfunc,
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

    pub fn invoke(&mut self, args: RuntimeArgs) -> Result<Option<RuntimeValue>, Trap> {
        println!("{:?}.invoke({:?}) ...", self, args);
        let ret = self.hostfunc.call_mut((args,));
        println!("{:?}.invoke(...) -> {:?}", self, ret);
        Ok(ret)
    }
}
