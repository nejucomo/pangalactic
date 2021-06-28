#[derive(Debug, derive_more::From)]
pub struct Key(blake3::Hash);

impl Key {
    pub fn b64(&self) -> String {
        crate::b64::encode(self.0.as_bytes())
    }
}

impl PartialEq for Key {
    fn eq(&self, other: &Key) -> bool {
        self.0.eq(&other.0)
    }
}

impl Eq for Key {}
