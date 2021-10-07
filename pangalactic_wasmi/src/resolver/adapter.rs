use crate::{FromGuestArgs, HostFunc, IntoGuestReturn};
use wasmi::{RuntimeArgs, RuntimeValue, Signature, Trap};

pub(crate) trait HostFuncAdapter<V> {
    fn name(&self) -> String;
    fn signature(&self) -> Signature;
    fn invoke(&self, vm: &mut V, rta: RuntimeArgs<'_>) -> Result<Option<RuntimeValue>, Trap>;
}

impl<V, T> HostFuncAdapter<V> for T
where
    T: HostFunc<V>,
{
    fn name(&self) -> String {
        <Self as HostFunc<V>>::name(self)
    }

    fn signature(&self) -> Signature {
        Signature::new(
            <Self as HostFunc<V>>::Args::valuetypes(),
            <Self as HostFunc<V>>::Return::returntype(),
        )
    }

    fn invoke(&self, vm: &mut V, rta: RuntimeArgs<'_>) -> Result<Option<RuntimeValue>, Trap> {
        let args = <Self as HostFunc<V>>::Args::from_guest_args(rta)?;
        let ret = <Self as HostFunc<V>>::invoke(self, vm, args)?;
        ret.into_guest_return(vm)
    }
}
