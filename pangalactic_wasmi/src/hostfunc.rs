use crate::{FromGuestArgs, IntoGuestReturn};
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
pub(crate) struct HostFn1<V, F, A, R>
where
    F: Fn(&mut V, A) -> Result<R, Trap>,
    A: FromGuestArgs,
    R: IntoGuestReturn,
{
    f: F,
    phantom: std::marker::PhantomData<(V, A, R)>,
}

impl<V, F, A, R> From<F> for HostFn1<V, F, A, R>
where
    F: Fn(&mut V, A) -> Result<R, Trap>,
    A: FromGuestArgs,
    R: IntoGuestReturn,
{
    fn from(f: F) -> Self {
        HostFn1 {
            f,
            phantom: std::marker::PhantomData,
        }
    }
}

impl<V, F, A, R> HostFunc<V> for HostFn1<V, F, A, R>
where
    F: Fn(&mut V, A) -> Result<R, Trap>,
    A: FromGuestArgs,
    R: IntoGuestReturn,
{
    type Args = A;
    type Return = R;

    fn name(&self) -> String {
        get_name::<F>()
    }

    fn invoke(&self, vm: &mut V, args: Self::Args) -> Result<Self::Return, Trap> {
        self.f.call((vm, args))
    }
}

fn get_name<T: Sized>() -> String {
    use convert_case::{Case, Casing};
    dbg!(std::any::type_name::<T>()
        .split("<")
        .next()
        .unwrap()
        .split("::")
        .last()
        .unwrap()
        .to_case(Case::Snake))
}

#[test]
fn get_name_from_generic() {
    struct FooBar<T>(T);

    assert_eq!(
        "foo_bar",
        get_name::<FooBar<std::marker::PhantomData<u64>>>()
    );
}
