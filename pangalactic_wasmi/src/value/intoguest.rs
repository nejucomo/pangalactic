use super::invalid_int;
use std::convert::TryFrom;
use wasmi::{RuntimeValue, Trap, ValueType};

pub trait IntoGuestValue: Sized {
    fn into_guest_type() -> ValueType;
    fn into_guest_value(self) -> Result<RuntimeValue, Trap>;
}

impl<T> IntoGuestValue for T
where
    i64: TryFrom<T>,
{
    fn into_guest_type() -> ValueType {
        ValueType::I64
    }

    fn into_guest_value(self) -> Result<RuntimeValue, Trap> {
        i64::try_from(self)
            .map_err(|_| invalid_int())
            .map(RuntimeValue::I64)
    }
}
