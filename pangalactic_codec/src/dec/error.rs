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
