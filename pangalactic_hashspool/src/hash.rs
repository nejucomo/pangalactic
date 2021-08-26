use blake3;
pub use blake3::OUT_LEN as HASH_LENGTH;
use std::fmt;

#[derive(Copy, Eq, PartialEq, Clone, Debug, derive_more::From)]
pub struct Hash(blake3::Hash);

impl Hash {
    pub fn as_bytes(&self) -> &[u8; HASH_LENGTH] {
        self.0.as_bytes()
    }
}

impl std::hash::Hash for Hash {
    fn hash<H>(&self, state: &mut H)
    where
        H: std::hash::Hasher,
    {
        self.0.hash(state)
    }
}

impl serde::Serialize for Hash {
    fn serialize<S>(&self, serializer: S) -> Result<S::Ok, S::Error>
    where
        S: serde::Serializer,
    {
        serializer.serialize_bytes(self.0.as_bytes())
    }
}

impl<'de> serde::Deserialize<'de> for Hash {
    fn deserialize<D>(deserializer: D) -> Result<Hash, D::Error>
    where
        D: serde::Deserializer<'de>,
    {
        deserializer.deserialize_bytes(HashVisitor)
    }
}

struct HashVisitor;

impl<'de> serde::de::Visitor<'de> for HashVisitor {
    type Value = Hash;

    fn expecting(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "a byte array containing {} bytes", HASH_LENGTH)
    }

    fn visit_bytes<E>(self, s: &[u8]) -> Result<Self::Value, E>
    where
        E: serde::de::Error,
    {
        if s.len() == HASH_LENGTH {
            let mut a: [u8; HASH_LENGTH] = [0u8; HASH_LENGTH];
            a.clone_from_slice(s);
            Ok(Hash::from(blake3::Hash::from(a)))
        } else {
            Err(serde::de::Error::invalid_length(s.len(), &self))
        }
    }
}
