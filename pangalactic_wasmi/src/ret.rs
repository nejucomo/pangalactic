use crate::IntoGuestValue;
use wasmi::{RuntimeValue, Trap, ValueType};

pub trait IntoGuestReturn {
    fn into_guest_return_type() -> Option<ValueType>;
    fn into_guest_return(self) -> Result<Option<RuntimeValue>, Trap>;
}

#[derive(Copy, Clone, Debug)]
pub struct Void;

impl IntoGuestReturn for Void {
    fn into_guest_return_type() -> Option<ValueType> {
        None
    }

    fn into_guest_return(self) -> Result<Option<RuntimeValue>, Trap> {
        Ok(None)
    }
}

impl<T> IntoGuestReturn for T
where
    T: IntoGuestValue,
{
    fn into_guest_return_type() -> Option<ValueType> {
        Some(Self::into_guest_type())
    }

    fn into_guest_return(self) -> Result<Option<RuntimeValue>, Trap> {
        self.into_guest_value().map(Some)
    }
}
