#[derive(Debug, derive_more::From)]
pub struct Key(blake3::Hash);

impl Key {
    pub fn b64(&self) -> String {
        crate::b64::encode(self.0.as_bytes())
    }
}
