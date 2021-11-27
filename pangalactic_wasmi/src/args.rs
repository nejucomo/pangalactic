use crate::FromGuestValue;
use wasmi::{RuntimeArgs, Trap, ValueType};

pub trait FromGuestArgs: Sized {
    fn valuetypes() -> Vec<ValueType>;
    fn from_guest_args(rta: RuntimeArgs<'_>) -> Result<Self, Trap>;
}

impl<T> FromGuestArgs for (T,)
where
    T: FromGuestValue,
{
    fn valuetypes() -> Vec<ValueType> {
        vec![T::from_guest_type()]
    }

    fn from_guest_args(rta: RuntimeArgs<'_>) -> Result<Self, Trap> {
        let rtar = rta.as_ref();
        if rtar.len() == 1 {
            let v = T::from_guest_value(rtar[0])?;
            Ok((v,))
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
...
*/
