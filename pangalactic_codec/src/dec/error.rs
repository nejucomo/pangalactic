use pangalactic_errorutil::into_std_error;

#[derive(Debug, derive_more::From, derive_more::Into)]
pub struct DecodeBytesError(serde_cbor::Error);

#[derive(Debug, derive_more::From)]
pub enum DecodeStringError {
    Base64(base64::DecodeError),
    Bytes(DecodeBytesError),
}

into_std_error!(DecodeBytesError, std::io::ErrorKind::InvalidData);
into_std_error!(DecodeStringError, std::io::ErrorKind::InvalidData);

use std::fmt;

impl fmt::Display for DecodeBytesError {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "{:?}", self)
    }
}

impl fmt::Display for DecodeStringError {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        use DecodeStringError::*;
        match self {
            Base64(e) => write!(f, "malformed base64: {:?}", e),
            Bytes(e) => write!(f, "malformed binary encoding: {}", e),
        }
    }
}
