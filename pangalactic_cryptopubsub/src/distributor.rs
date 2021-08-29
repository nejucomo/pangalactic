use crate::{Publication, PublicationContents};
use pangalactic_codec::{decode_bytes, DecodeBytesError};
use pangalactic_signpair::Verifier;

#[derive(Clone, Copy, Debug, derive_more::From)]
pub struct Distributor(Verifier);

#[derive(Debug, derive_more::From)]
pub enum UnwrapError {
    InvalidSignature(pangalactic_signpair::InvalidSignature),
    MalformedEncoding(DecodeBytesError),
}

impl Distributor {
    pub fn unwrap(&self, p: &Publication) -> Result<PublicationContents, UnwrapError> {
        let bytes = self.0.verify(p.as_ref())?;
        let contents = decode_bytes(&bytes[..])?;
        Ok(contents)
    }
}
