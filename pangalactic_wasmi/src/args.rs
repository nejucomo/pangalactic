use crate::FromRuntimeValue;
use wasmi::{RuntimeArgs, Trap, ValueType};

pub trait FromRuntimeArgs: Sized {
    fn valuetypes() -> Vec<ValueType>;
    fn from_runtime_args(rta: RuntimeArgs<'_>) -> Result<Self, Trap>;
}

impl<T> FromRuntimeArgs for T
where
    T: FromRuntimeValue,
{
    fn valuetypes() -> Vec<ValueType> {
        vec![Self::valuetype()]
    }

    fn from_runtime_args(rta: RuntimeArgs<'_>) -> Result<Self, Trap> {
        let rtar = rta.as_ref();
        if rtar.len() == 1 {
            Self::from_runtime_value(rtar[0])
        } else {
            use wasmi::TrapKind::UnexpectedSignature;

            Err(Trap::new(UnexpectedSignature))
        }
    }
}

impl<T, U> FromRuntimeArgs for (T, U)
where
    T: FromRuntimeValue,
    U: FromRuntimeValue,
{
    fn valuetypes() -> Vec<ValueType> {
        vec![T::valuetype(), U::valuetype()]
    }

    fn from_runtime_args(rta: RuntimeArgs<'_>) -> Result<Self, Trap> {
        let rtar = rta.as_ref();
        if rtar.len() == 2 {
            let t = T::from_runtime_value(rtar[0])?;
            let u = U::from_runtime_value(rtar[1])?;
            Ok((t, u))
        } else {
            use wasmi::TrapKind::UnexpectedSignature;

            Err(Trap::new(UnexpectedSignature))
        }
    }
}
