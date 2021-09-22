use wasmi::{RuntimeValue, Trap, ValueType};

pub trait FromRuntimeValue: Sized {
    fn valuetype() -> ValueType;
    fn from_runtime_value(rtv: RuntimeValue) -> Result<Self, Trap>;
}
