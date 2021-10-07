use std::convert::TryFrom;
use wasmi::{RuntimeValue, Trap, ValueType};

pub trait FromGuestValue<V>: Sized {
    fn valuetype() -> ValueType;
    fn from_guest_value(vm: &V, rtv: RuntimeValue) -> Result<Self, Trap>;
}

impl<V> FromGuestValue<V> for i64 {
    fn valuetype() -> ValueType {
        ValueType::I64
    }

    fn from_guest_value(_vm: &V, rtv: RuntimeValue) -> Result<Self, Trap> {
        match rtv {
            RuntimeValue::I64(i) => Ok(i),
            _ => Err(invalid_int()),
        }
    }
}

impl<V> FromGuestValue<V> for usize {
    fn valuetype() -> ValueType {
        ValueType::I64
    }

    fn from_guest_value(vm: &V, rtv: RuntimeValue) -> Result<Self, Trap> {
        let i = i64::from_guest_value(vm, rtv)?;
        usize::try_from(i).map_err(|_| invalid_int())
    }
}

fn invalid_int() -> Trap {
    Trap::new(wasmi::TrapKind::InvalidConversionToInt)
}
