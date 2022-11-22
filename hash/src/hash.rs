#[derive(Clone, Debug, Eq, PartialEq, Hash)]
pub struct Hash(blake3::Hash);

impl Hash {
    pub fn of<T>(t: T) -> Hash
    where
        T: AsRef<[u8]>,
    {
        use std::io::Write;

        let mut hasher = crate::Hasher::default();
        hasher.write_all(t.as_ref()).unwrap();
        hasher.finalize()
    }

    pub(crate) fn wrap(b3h: blake3::Hash) -> Self {
        Hash(b3h)
    }
}
