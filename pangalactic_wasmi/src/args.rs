use crate::FromGuestValue;
use wasmi::{RuntimeArgs, Trap, ValueType};

pub trait FromGuestArgs: Sized {
    fn valuetypes() -> Vec<ValueType>;
    fn from_guest_args(rta: RuntimeArgs<'_>) -> Result<Self, Trap>;
}

impl FromGuestArgs for () {
    fn valuetypes() -> Vec<ValueType> {
        vec![]
    }

    fn from_guest_args(rta: RuntimeArgs<'_>) -> Result<Self, Trap> {
        let rtar = rta.as_ref();
        if rtar.len() == 0 {
            Ok(())
        } else {
            use wasmi::TrapKind::UnexpectedSignature;

            Err(Trap::new(UnexpectedSignature))
        }
    }
}

impl<A> FromGuestArgs for (A,)
where
    A: FromGuestValue,
{
    fn valuetypes() -> Vec<ValueType> {
        vec![A::from_guest_type()]
    }

    fn from_guest_args(rta: RuntimeArgs<'_>) -> Result<Self, Trap> {
        let rtar = rta.as_ref();
        if rtar.len() == 1 {
            let v = A::from_guest_value(rtar[0])?;
            Ok((v,))
        } else {
            use wasmi::TrapKind::UnexpectedSignature;

            Err(Trap::new(UnexpectedSignature))
        }
    }
}

impl<A, B> FromGuestArgs for (A, B)
where
    A: FromGuestValue,
    B: FromGuestValue,
{
    fn valuetypes() -> Vec<ValueType> {
        vec![A::from_guest_type(), B::from_guest_type()]
    }

    fn from_guest_args(rta: RuntimeArgs<'_>) -> Result<Self, Trap> {
        let rtar = rta.as_ref();
        if rtar.len() == 2 {
            let a = A::from_guest_value(rtar[0])?;
            let b = B::from_guest_value(rtar[1])?;
            Ok((a, b))
        } else {
            use wasmi::TrapKind::UnexpectedSignature;

            Err(Trap::new(UnexpectedSignature))
        }
    }
}

impl<A, B, C> FromGuestArgs for (A, B, C)
where
    A: FromGuestValue,
    B: FromGuestValue,
    C: FromGuestValue,
{
    fn valuetypes() -> Vec<ValueType> {
        vec![
            A::from_guest_type(),
            B::from_guest_type(),
            C::from_guest_type(),
        ]
    }

    fn from_guest_args(rta: RuntimeArgs<'_>) -> Result<Self, Trap> {
        let rtar = rta.as_ref();
        if rtar.len() == 3 {
            let a = A::from_guest_value(rtar[0])?;
            let b = B::from_guest_value(rtar[1])?;
            let c = C::from_guest_value(rtar[2])?;
            Ok((a, b, c))
        } else {
            use wasmi::TrapKind::UnexpectedSignature;

            Err(Trap::new(UnexpectedSignature))
        }
    }
}

impl<A, B, C, D> FromGuestArgs for (A, B, C, D)
where
    A: FromGuestValue,
    B: FromGuestValue,
    C: FromGuestValue,
    D: FromGuestValue,
{
    fn valuetypes() -> Vec<ValueType> {
        vec![
            A::from_guest_type(),
            B::from_guest_type(),
            C::from_guest_type(),
            D::from_guest_type(),
        ]
    }

    fn from_guest_args(rta: RuntimeArgs<'_>) -> Result<Self, Trap> {
        let rtar = rta.as_ref();
        if rtar.len() == 4 {
            let a = A::from_guest_value(rtar[0])?;
            let b = B::from_guest_value(rtar[1])?;
            let c = C::from_guest_value(rtar[2])?;
            let d = D::from_guest_value(rtar[3])?;
            Ok((a, b, c, d))
        } else {
            use wasmi::TrapKind::UnexpectedSignature;

            Err(Trap::new(UnexpectedSignature))
        }
    }
}
