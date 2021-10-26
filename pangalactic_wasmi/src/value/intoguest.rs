use super::invalid_int;
use crate::HasGuestType;
use std::convert::TryFrom;
use wasmi::{RuntimeValue, Trap};

pub trait IntoGuestValue: Sized + HasGuestType {
    fn into_guest_value(self) -> Result<RuntimeValue, Trap>;
}

impl IntoGuestValue for i64 {
    fn into_guest_value(self) -> Result<RuntimeValue, Trap> {
        Ok(RuntimeValue::I64(self))
    }
}

impl IntoGuestValue for usize {
    fn into_guest_value(self) -> Result<RuntimeValue, Trap> {
        i64::try_from(self)
            .map_err(|_| invalid_int())
            .and_then(|i| i.into_guest_value())
    }
}
