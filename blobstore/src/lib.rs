// TODO: This design doesn't address GC / data availability.

mod blobstore;
mod writer;

pub use self::blobstore::BlobStore;
pub use self::writer::Writer;
