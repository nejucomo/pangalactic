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

#[derive(Debug, thiserror::Error)]
#[error("invalid path {input:?}: {reason}")]
pub struct PathError {
    pub input: String,
    pub reason: InvalidPath,
}

aliri_braid::from_infallible!(PathError);

#[derive(Debug, thiserror::Error)]
pub enum InvalidPath {
    #[error("empty")]
    Empty,
    #[error("invalid UTF8")]
    UTF8,
    #[error("contains invalid name: {0}")]
    Name(NameError),
}
