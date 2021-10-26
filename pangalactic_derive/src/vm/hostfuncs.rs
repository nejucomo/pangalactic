use crate::vm::{LinkHandle, VirtualMachine};
use pangalactic_node::Kind;
use pangalactic_store::Store;
use pangalactic_wasmi::{HostFuncResolver, IntoGuestValue};
use wasmi::{RuntimeValue, Trap, ValueType};

pub fn new_hostfunc_resolver<S>() -> HostFuncResolver<VirtualMachine<S>>
where
    S: Store + 'static,
{
    let mut hfr = HostFuncResolver::new();
    hfr.add_host_fn1(link_kind);
    hfr
}

fn link_kind<S>(vm: &mut VirtualMachine<S>, handle: LinkHandle<S>) -> Result<GuestKind, Trap>
where
    S: Store,
{
    let link = vm.links.get(handle)?;
    Ok(GuestKind(link.kind))
}

#[derive(Debug, Copy, Clone)]
pub struct GuestKind(Kind);

impl IntoGuestValue for GuestKind {
    fn into_guest_type() -> ValueType {
        ValueType::I32
    }

    fn into_guest_value(self) -> Result<RuntimeValue, Trap> {
        Ok(RuntimeValue::I32(match self.0 {
            Kind::File => 0,
            Kind::Dir => 1,
        }))
    }
}
