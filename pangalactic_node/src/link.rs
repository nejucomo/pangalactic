#[cfg(test)]
mod tests;

use crate::Kind;
use serde::{Deserialize, Serialize};

#[derive(Clone, Debug, Eq, PartialEq, Deserialize, Serialize)]
pub struct Link<K> {
    pub kind: Kind,
    pub key: K,
}

impl<K> Link<K>
where
    K: std::fmt::Debug,
{
    pub fn get_key(&self, kind: Kind) -> std::io::Result<&K> {
        if self.kind == kind {
            Ok(&self.key)
        } else {
            use pangalactic_errorutil::io_error;

            Err(io_error!(
                std::io::ErrorKind::InvalidInput,
                "Incorrect Link type, expected {:?}, found {:?}",
                kind,
                self
            ))
        }
    }

    pub fn get_file_key(&self) -> std::io::Result<&K> {
        self.get_key(Kind::File)
    }

    pub fn get_dir_key(&self) -> std::io::Result<&K> {
        self.get_key(Kind::Dir)
    }
}

use std::fmt;

impl<K> fmt::Display for Link<K>
where
    K: Serialize,
{
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "{}", pangalactic_codec::encode_string(self))
    }
}

impl<K> std::str::FromStr for Link<K>
where
    K: serde::de::DeserializeOwned,
{
    type Err = pangalactic_codec::DecodeStringError;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        pangalactic_codec::decode_string(s)
    }
}
