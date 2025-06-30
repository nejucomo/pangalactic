#[derive(Debug, thiserror::Error)]
#[error("invalid name {input:?}: {reason}")]
pub struct NameError {
    pub input: String,
    pub reason: InvalidName,
}

aliri_braid::from_infallible!(NameError);

#[derive(Debug, thiserror::Error)]
pub enum InvalidName {
    #[error("empty")]
    Empty,
    #[error("contains separator '/'")]
    ContainsSeparator,
    #[error("invalid UTF8")]
    UTF8,
}
