use crate::vm::{LinkHandle, VirtualMachine};
use pangalactic_node::Kind;
use pangalactic_store::Store;
use pangalactic_wasmi::{HostFunc, IntoGuestReturn};
use std::marker::PhantomData;
use wasmi::{RuntimeValue, Trap, ValueType};

pub(crate) struct LinkKind<S>(PhantomData<S>)
where
    S: Store;

impl<S> LinkKind<S>
where
    S: Store,
{
    pub(crate) fn new() -> LinkKind<S> {
        LinkKind(PhantomData)
    }
}

impl<S> HostFunc<VirtualMachine<S>> for LinkKind<S>
where
    S: Store,
{
    type Args = LinkHandle<S>;
    type Return = GuestKind;

    fn invoke(&self, vm: &mut VirtualMachine<S>, handle: LinkHandle<S>) -> Result<GuestKind, Trap> {
        let link = vm.links.get(handle)?;
        Ok(GuestKind(link.kind))
    }
}

#[derive(Debug, Copy, Clone)]
pub struct GuestKind(Kind);

impl IntoGuestReturn for GuestKind {
    fn returntype() -> Option<ValueType> {
        Some(ValueType::I32)
    }

    fn into_guest_return(self) -> Result<Option<RuntimeValue>, Trap> {
        Ok(Some(RuntimeValue::I32(match self.0 {
            Kind::File => 0,
            Kind::Dir => 1,
        })))
    }
}
