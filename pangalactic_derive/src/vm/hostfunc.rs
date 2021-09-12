use crate::vm::VirtualMachine;
use enum_iterator::IntoEnumIterator;
use num_derive::FromPrimitive;
use pangalactic_store::Store;
use wasmi::ValueType::I64;
use wasmi::{RuntimeArgs, RuntimeValue, Signature, Trap};

#[derive(Copy, Clone, Debug, IntoEnumIterator, FromPrimitive)]
pub enum HostFunc {
    LinkType,
}

use HostFunc::*;

impl HostFunc {
    pub fn name(self) -> &'static str {
        match self {
            LinkType => "link_type",
        }
    }

    pub fn signature(self) -> Signature {
        match self {
            LinkType => Signature::new(&[I64][..], Some(I64)),
        }
    }

    pub fn invoke<'a, S>(
        self,
        _vm: &mut VirtualMachine<'a, S>,
        args: RuntimeArgs<'_>,
    ) -> Result<Option<RuntimeValue>, Trap>
    where
        S: Store,
    {
        todo!("{:?}.invoke(..., {:?})", self, args);
    }
}
