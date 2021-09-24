use crate::{FromGuestArgs, IntoGuestReturn};
use wasmi::Trap;

pub trait HostFunc<V> {
    type Args: FromGuestArgs<V>;
    type Return: IntoGuestReturn<V>;

    fn name(&self) -> &'static str;
    fn invoke(&self, vm: &mut V, args: Self::Args) -> Result<Self::Return, Trap>;
}
