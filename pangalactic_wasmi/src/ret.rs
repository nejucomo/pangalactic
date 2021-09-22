use wasmi::{RuntimeValue, Trap, ValueType};

pub trait IntoGuestReturn<V> {
    fn returntype() -> Option<ValueType>;

    fn into_guest_return(self, vm: &V) -> Result<Option<RuntimeValue>, Trap>;
}

impl<V> IntoGuestReturn<V> for () {
    fn returntype() -> Option<ValueType> {
        None
    }

    fn into_guest_return(self, _vm: &V) -> Result<Option<RuntimeValue>, Trap> {
        Ok(None)
    }
}
