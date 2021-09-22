use crate::FromGuestValue;
use wasmi::{RuntimeArgs, Trap, ValueType};

pub trait FromGuestArgs<V>: Sized {
    fn valuetypes() -> Vec<ValueType>;
    fn from_guest_args(vm: &V, rta: RuntimeArgs<'_>) -> Result<Self, Trap>;
}

impl<V, T> FromGuestArgs<V> for T
where
    T: FromGuestValue<V>,
{
    fn valuetypes() -> Vec<ValueType> {
        vec![Self::valuetype()]
    }

    fn from_guest_args(vm: &V, rta: RuntimeArgs<'_>) -> Result<Self, Trap> {
        let rtar = rta.as_ref();
        if rtar.len() == 1 {
            Self::from_guest_value(vm, rtar[0])
        } else {
            use wasmi::TrapKind::UnexpectedSignature;

            Err(Trap::new(UnexpectedSignature))
        }
    }
}

/* Overlapping instances failure after introducing <V>:
impl<V, T, U> FromGuestArgs<V> for (T, U)
where
    T: FromGuestValue<V>,
    U: FromGuestValue<V>,
{
    fn valuetypes() -> Vec<ValueType> {
        vec![T::valuetype(), U::valuetype()]
    }

    fn from_guest_args(vm: &V, rta: RuntimeArgs<'_>) -> Result<Self, Trap> {
        let rtar = rta.as_ref();
        if rtar.len() == 2 {
            let t = T::from_guest_value(vm, rtar[0])?;
            let u = U::from_guest_value(vm, rtar[1])?;
            Ok((t, u))
        } else {
            use wasmi::TrapKind::UnexpectedSignature;

            Err(Trap::new(UnexpectedSignature))
        }
    }
}
*/
