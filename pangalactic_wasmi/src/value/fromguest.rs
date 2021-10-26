use super::invalid_int;
use std::convert::TryFrom;
use wasmi::{RuntimeValue, Trap, ValueType};

pub trait FromGuestValue: Sized {
    fn from_guest_type() -> ValueType;
    fn from_guest_value(rtv: RuntimeValue) -> Result<Self, Trap>;
}

impl<T> FromGuestValue for T
where
    T: TryFrom<i64>,
{
    fn from_guest_type() -> ValueType {
        ValueType::I64
    }

    fn from_guest_value(rtv: RuntimeValue) -> Result<Self, Trap> {
        let i = match rtv {
            RuntimeValue::I64(i) => Ok(i),
            _ => Err(invalid_int()),
        }?;

        Self::try_from(i).map_err(|_| invalid_int())
    }
}
