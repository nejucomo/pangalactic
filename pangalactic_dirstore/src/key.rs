use std::fmt;

#[derive(Copy, Clone, Debug, derive_more::From)]
pub struct Key(blake3::Hash);

impl pangalactic_store::StoreKey for Key {}

impl PartialEq for Key {
    fn eq(&self, other: &Key) -> bool {
        self.0.eq(&other.0)
    }
}

impl Eq for Key {}

impl serde::Serialize for Key {
    fn serialize<S>(&self, serializer: S) -> Result<S::Ok, S::Error>
    where
        S: serde::Serializer,
    {
        serializer.serialize_bytes(self.0.as_bytes())
    }
}

impl<'de> serde::Deserialize<'de> for Key {
    fn deserialize<D>(deserializer: D) -> Result<Key, D::Error>
    where
        D: serde::Deserializer<'de>,
    {
        deserializer.deserialize_bytes(KeyVisitor)
    }
}

struct KeyVisitor;

impl<'de> serde::de::Visitor<'de> for KeyVisitor {
    type Value = Key;

    fn expecting(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "a byte array containing {} bytes", blake3::OUT_LEN)
    }

    fn visit_bytes<E>(self, s: &[u8]) -> Result<Self::Value, E>
    where
        E: serde::de::Error,
    {
        if s.len() == blake3::OUT_LEN {
            let mut a: [u8; blake3::OUT_LEN] = [0u8; blake3::OUT_LEN];
            a.clone_from_slice(s);
            Ok(Key::from(blake3::Hash::from(a)))
        } else {
            Err(serde::de::Error::invalid_length(s.len(), &self))
        }
    }
}
