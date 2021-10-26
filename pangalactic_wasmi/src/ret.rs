use crate::IntoGuestValue;
use wasmi::{RuntimeValue, Trap, ValueType};

pub trait IntoGuestReturn {
    fn returntype() -> Option<ValueType>;

    fn into_guest_return(self) -> Result<Option<RuntimeValue>, Trap>;
}

impl IntoGuestReturn for () {
    fn returntype() -> Option<ValueType> {
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
    fn returntype() -> Option<ValueType> {
        Some(Self::valuetype())
    }

    fn into_guest_return(self) -> Result<Option<RuntimeValue>, Trap> {
        self.into_guest_value().map(Some)
    }
}
