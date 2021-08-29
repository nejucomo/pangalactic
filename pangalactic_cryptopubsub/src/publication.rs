use serde::{Deserialize, Serialize};

#[derive(Debug, derive_more::From)]
pub struct Publication(Vec<u8>);

impl AsRef<[u8]> for Publication {
    fn as_ref(&self) -> &[u8] {
        self.0.as_ref()
    }
}

#[derive(Debug, PartialEq, Eq, Deserialize, Serialize)]
pub struct PublicationContents {
    pub sequence: u64,
    pub data: Vec<u8>,
}
