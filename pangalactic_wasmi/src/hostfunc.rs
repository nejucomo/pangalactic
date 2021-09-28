use crate::{FromGuestArgs, IntoGuestReturn};
use wasmi::Trap;

pub trait HostFunc<V>: Sized {
    type Args: FromGuestArgs<V>;
    type Return: IntoGuestReturn<V>;

    fn name(&self) -> String {
        get_name::<Self>()
    }

    fn invoke(&self, vm: &mut V, args: Self::Args) -> Result<Self::Return, Trap>;
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
