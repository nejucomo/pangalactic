use crate::vm::VirtualMachine;
use pangalactic_node::Kind;
use pangalactic_store::Store;
use pangalactic_wasmi::IntoGuestReturn;
use wasmi::{RuntimeValue, Trap, ValueType};

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
