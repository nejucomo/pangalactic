use crate::{FromRuntimeArgs, IntoRuntimeReturn};
use wasmi::{RuntimeArgs, RuntimeValue, Signature, Trap};

pub trait HostFunc {
    type Args: FromRuntimeArgs;
    type Return: IntoRuntimeReturn;

    fn name(&self) -> &'static str;
    fn invoke(&self, args: Self::Args) -> Result<Self::Return, Trap>;
}

pub(crate) trait HostFuncAdapter {
    fn name(&self) -> &'static str;
    fn signature(&self) -> Signature;
    fn invoke(&self, rta: RuntimeArgs<'_>) -> Result<Option<RuntimeValue>, Trap>;
}

impl<T> HostFuncAdapter for T
where
    T: HostFunc,
{
    fn name(&self) -> &'static str {
        <Self as HostFunc>::name(self)
    }

    fn signature(&self) -> Signature {
        Signature::new(
            <Self as HostFunc>::Args::valuetypes(),
            <Self as HostFunc>::Return::returntype(),
        )
    }

    fn invoke(&self, rta: RuntimeArgs<'_>) -> Result<Option<RuntimeValue>, Trap> {
        let args = <Self as HostFunc>::Args::from_runtime_args(rta)?;
        let ret = <Self as HostFunc>::invoke(self, args)?;
        ret.into_runtime_return()
    }
}
