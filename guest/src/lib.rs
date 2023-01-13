#[macro_use]
mod log;

pub mod bindings;
mod bytereader;
mod bytewriter;
mod directoryreader;
mod directorywriter;
mod link;
pub(crate) mod ptr;
mod reader;
mod schemata;

pub use self::bytereader::ByteReader;
pub use self::bytewriter::{write_bytes, ByteWriter};
pub use self::directoryreader::DirectoryReader;
pub use self::directorywriter::DirectoryWriter;
pub use self::link::Link;
pub use self::log::log_str;
pub use self::reader::Reader;
pub use self::schemata::{Attestation, Directory, Plan};
pub use dagwasm_guest_procmacro::define_derive;
pub use dagwasm_linkkind::LinkKind;
pub use dagwasm_primitives as prim;
