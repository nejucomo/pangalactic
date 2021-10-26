use serde::{Deserialize, Serialize};
use std::convert::TryFrom;

#[cfg(test)]
mod tests;

#[derive(Clone, Copy, Debug, Eq, PartialEq, Deserialize, Serialize)]
pub enum Kind {
    File,
    Dir,
}

#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub struct UnknownKindEncoding(pub usize);

impl TryFrom<usize> for Kind {
    type Error = UnknownKindEncoding;

    fn try_from(u: usize) -> Result<Kind, UnknownKindEncoding> {
        match u {
            0 => Ok(Kind::File),
            1 => Ok(Kind::Dir),
            _ => Err(UnknownKindEncoding(u)),
        }
    }
}

impl From<Kind> for usize {
    fn from(k: Kind) -> usize {
        match k {
            Kind::File => 0,
            Kind::Dir => 1,
        }
    }
}
