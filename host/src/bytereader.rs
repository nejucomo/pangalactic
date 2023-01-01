use dagwasm_dir::Name;
use std::io::Cursor;

#[derive(Debug)]
pub(crate) struct ByteReader(Cursor<Vec<u8>>);

impl From<Name> for ByteReader {
    fn from(name: Name) -> Self {
        ByteReader(Cursor::new(name.into_bytes()))
    }
}
