use crate::{FromGuestValue, IntoGuestValue};
use std::convert::TryFrom;
use std::fmt;
use std::marker::PhantomData;
use wasmi::{RuntimeValue, Trap, ValueType};

pub struct Handle<T>(usize, PhantomData<T>);

impl<T> Copy for Handle<T> {}

impl<T> Clone for Handle<T> {
    fn clone(&self) -> Self {
        Handle(self.0, PhantomData)
    }
}

impl<T> fmt::Debug for Handle<T> {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "{}({})", std::any::type_name::<Self>(), self.0)
    }
}

impl<T> From<usize> for Handle<T> {
    fn from(u: usize) -> Self {
        Handle(u, PhantomData)
    }
}

impl<T> From<Handle<T>> for usize {
    fn from(h: Handle<T>) -> usize {
        h.0
    }
}

impl<T> FromGuestValue for Handle<T> {
    fn from_guest_type() -> ValueType {
        usize::from_guest_type()
    }

    fn from_guest_value(rtv: RuntimeValue) -> Result<Self, Trap> {
        let u = usize::from_guest_value(rtv)?;
        Ok(Handle::from(u))
    }
}

impl<T> IntoGuestValue for Handle<T> {
    fn into_guest_type() -> ValueType {
        usize::from_guest_type()
    }

    fn into_guest_value(self) -> Result<RuntimeValue, Trap> {
        self.0.into_guest_value()
    }
}

impl<T> TryFrom<Handle<T>> for RuntimeValue {
    type Error = wasmi::Error;

    fn try_from(h: Handle<T>) -> Result<RuntimeValue, Self::Error> {
        let i = i64::try_from(h.0).map_err(|e| {
            wasmi::Error::Instantiation(format!(
                "Overflow error when converting {:?} into runtime value: {:?}",
                h, e
            ))
        })?;
        Ok(RuntimeValue::from(i))
    }
}

impl<T> TryFrom<RuntimeValue> for Handle<T> {
    type Error = wasmi::Error;

    fn try_from(rtv: RuntimeValue) -> Result<Self, Self::Error> {
        let mk_err = |issue| {
            wasmi::Error::Instantiation(format!(
                "{} when converting {:?} into {}",
                issue,
                rtv,
                std::any::type_name::<Self>()
            ))
        };

        let i = match rtv {
            RuntimeValue::I64(i) => i,
            _ => Err(mk_err("incorrect type"))?,
        };

        let u = usize::try_from(i).map_err(|_| mk_err("overflow"))?;
        Ok(Handle(u, PhantomData))
    }
}
