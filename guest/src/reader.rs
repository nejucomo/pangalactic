use crate::{ByteReader, DirectoryReader};

#[derive(Debug)]
pub enum Reader {
    File(ByteReader),
    Dir(DirectoryReader),
}
