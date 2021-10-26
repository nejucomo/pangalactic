use super::invalid_int;
use crate::HasGuestType;
use std::convert::TryFrom;
use wasmi::{RuntimeValue, Trap};

pub trait FromGuestValue: Sized + HasGuestType {
    fn from_guest_value(rtv: RuntimeValue) -> Result<Self, Trap>;
}

impl FromGuestValue for i64 {
    fn from_guest_value(rtv: RuntimeValue) -> Result<Self, Trap> {
        match rtv {
            RuntimeValue::I64(i) => Ok(i),
            _ => Err(invalid_int()),
        }
    }
}

impl FromGuestValue for usize {
    fn from_guest_value(rtv: RuntimeValue) -> Result<Self, Trap> {
        let i = i64::from_guest_value(rtv)?;
        usize::try_from(i).map_err(|_| invalid_int())
    }
}
