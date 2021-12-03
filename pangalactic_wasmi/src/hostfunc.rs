mod fnwrappers;

use crate::{FromGuestArgs, IntoGuestReturn};
use wasmi::Trap;

pub(crate) use self::fnwrappers::{HostFn0, HostFn1, HostFn2, HostFn3, HostFn4};

pub trait HostFunc<V>: Sized {
    type Args: FromGuestArgs;
    type Return: IntoGuestReturn;

    fn name(&self) -> String {
        get_name::<Self>()
    }

    fn invoke(&self, vm: &mut V, args: Self::Args) -> Result<Self::Return, Trap>;
}

pub(crate) fn get_name<T: Sized>() -> String {
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
