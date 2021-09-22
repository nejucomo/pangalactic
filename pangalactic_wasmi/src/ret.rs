use wasmi::{RuntimeValue, Trap, ValueType};

pub trait IntoRuntimeReturn {
    fn returntype() -> Option<ValueType>;

    fn into_runtime_return(self) -> Result<Option<RuntimeValue>, Trap>;
}

impl IntoRuntimeReturn for () {
    fn returntype() -> Option<ValueType> {
        None
    }

    fn into_runtime_return(self) -> Result<Option<RuntimeValue>, Trap> {
        Ok(None)
    }
}
