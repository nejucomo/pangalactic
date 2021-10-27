use crate::prim;

#[derive(Debug, PartialEq, Eq, derive_more::From)]
pub struct ReadHandle(prim::Read);

impl ReadHandle {}
