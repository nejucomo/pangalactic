use crate::vm::VirtualMachine;
use pangalactic_node::Kind;
use pangalactic_nodestore::LinkFor;
use pangalactic_store::Store;
use pangalactic_wasmi::HostFunc;
use std::marker::PhantomData;
use wasmi::Trap;

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

    fn invoke(&self, _vm: &mut VirtualMachine<'a, S>, args: LinkFor<S>) -> Result<Kind, Trap> {
        Ok(args.kind)
    }
}
