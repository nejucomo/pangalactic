use crate::vm::{LinkHandle, VirtualMachine};
use pangalactic_node::Kind;
use pangalactic_nodestore::LinkFor;
use pangalactic_store::Store;
use pangalactic_wasmi::{FromGuestValue, HostFunc, IntoGuestReturn};
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

impl<'a, S> HostFunc<VirtualMachine<'a, S>> for LinkKind<S>
where
    S: Store,
{
    type Args = LinkFor<S>;
    type Return = Kind;

    fn name(&self) -> &'static str {
        "link_kind"
    }

    fn invoke(&self, _vm: &mut VirtualMachine<'a, S>, args: LinkFor<S>) -> Result<Kind, Trap> {
        Ok(args.kind)
    }
}

impl<'a, S> FromGuestValue<VirtualMachine<'a, S>> for LinkFor<S>
where
    S: Store,
{
    fn valuetype() -> ValueType {
        ValueType::I64
    }

    fn from_guest_value(vm: &VirtualMachine<'a, S>, rtv: RuntimeValue) -> Result<Self, Trap> {
        let u = usize::from_guest_value(vm, rtv)?;
        let handle = LinkHandle::<S>::from(u);
        let link = vm.links.get(handle)?;
        Ok(link.clone())
    }
}

impl<'a, S> IntoGuestReturn<VirtualMachine<'a, S>> for Kind
where
    S: Store,
{
    fn returntype() -> Option<ValueType> {
        Some(ValueType::I32)
    }

    fn into_guest_return(self, _vm: &VirtualMachine<'a, S>) -> Result<Option<RuntimeValue>, Trap> {
        Ok(Some(RuntimeValue::I32(match self {
            Kind::File => 0,
            Kind::Dir => 1,
        })))
    }
}
