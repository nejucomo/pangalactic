use crate::prim;

#[derive(Debug, PartialEq, Eq, derive_more::From)]
pub struct BufReaderHandle(prim::BufReaderHandle);

impl BufReaderHandle {}
