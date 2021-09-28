use crate::vm::{LinkHandle, VirtualMachine};
use pangalactic_node::Kind;
use pangalactic_nodestore::LinkFor;
use pangalactic_store::Store;
use pangalactic_wasmi::{FromGuestValue, IntoGuestReturn};
use wasmi::{RuntimeValue, Trap, ValueType};

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
