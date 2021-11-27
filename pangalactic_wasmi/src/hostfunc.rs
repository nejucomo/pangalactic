use crate::{FromGuestArgs, FromGuestValue, IntoGuestReturn};
use wasmi::Trap;

pub trait HostFunc<V>: Sized {
    type Args: FromGuestArgs;
    type Return: IntoGuestReturn;

    fn name(&self) -> String {
        get_name::<Self>()
    }

    fn invoke(&self, vm: &mut V, args: Self::Args) -> Result<Self::Return, Trap>;
}

// Wrap functions directly for dependent-crate ergonomics:
pub(crate) struct HostFn0<V, F, R, E>
where
    F: Fn(&mut V) -> Result<R, E>,
    R: IntoGuestReturn,
    Trap: From<E>,
{
    f: F,
    phantom: std::marker::PhantomData<(V, R, E)>,
}

impl<V, F, R, E> From<F> for HostFn0<V, F, R, E>
where
    F: Fn(&mut V) -> Result<R, E>,
    R: IntoGuestReturn,
    Trap: From<E>,
{
    fn from(f: F) -> Self {
        HostFn0 {
            f,
            phantom: std::marker::PhantomData,
        }
    }
}

impl<V, F, R, E> HostFunc<V> for HostFn0<V, F, R, E>
where
    F: Fn(&mut V) -> Result<R, E>,
    R: IntoGuestReturn,
    Trap: From<E>,
{
    type Args = ();
    type Return = R;

    fn name(&self) -> String {
        get_name::<F>()
    }

    fn invoke(&self, vm: &mut V, _args: ()) -> Result<Self::Return, Trap> {
        self.f.call((vm,)).map_err(|e: E| Trap::from(e))
    }
}

// HostFn1
pub(crate) struct HostFn1<V, F, A, R, E>
where
    F: Fn(&mut V, A) -> Result<R, E>,
    A: FromGuestValue,
    R: IntoGuestReturn,
    Trap: From<E>,
{
    f: F,
    phantom: std::marker::PhantomData<(V, A, R, E)>,
}

impl<V, F, A, R, E> From<F> for HostFn1<V, F, A, R, E>
where
    F: Fn(&mut V, A) -> Result<R, E>,
    A: FromGuestValue,
    R: IntoGuestReturn,
    Trap: From<E>,
{
    fn from(f: F) -> Self {
        HostFn1 {
            f,
            phantom: std::marker::PhantomData,
        }
    }
}

impl<V, F, A, R, E> HostFunc<V> for HostFn1<V, F, A, R, E>
where
    F: Fn(&mut V, A) -> Result<R, E>,
    A: FromGuestValue,
    R: IntoGuestReturn,
    Trap: From<E>,
{
    type Args = (A,);
    type Return = R;

    fn name(&self) -> String {
        get_name::<F>()
    }

    fn invoke(&self, vm: &mut V, (a,): (A,)) -> Result<Self::Return, Trap> {
        self.f.call((vm, a)).map_err(|e: E| Trap::from(e))
    }
}

fn get_name<T: Sized>() -> String {
    use convert_case::{Case, Casing};
    std::any::type_name::<T>()
        .split("<")
        .next()
        .unwrap()
        .split("::")
        .last()
        .unwrap()
        .to_case(Case::Snake)
}

#[test]
fn get_name_from_generic() {
    struct FooBar<T>(T);

    assert_eq!(
        "foo_bar",
        get_name::<FooBar<std::marker::PhantomData<u64>>>()
    );
}
