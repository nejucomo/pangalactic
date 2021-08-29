use crate::FileWriter;
use crate::ReadEntry;

use pangalactic_codec as codec;
use pangalactic_node::{Dir, Kind, Link};
use pangalactic_store::Store;
use std::io::Result as IOResult;

#[derive(Debug, derive_more::From)]
pub struct NodeStore<S>(S);

impl<S> NodeStore<S>
where
    S: Store,
{
    pub fn open_file_writer(&self) -> IOResult<FileWriter<<S as Store>::Writer>> {
        self.0.open_writer().map(FileWriter::from)
    }

    pub fn commit_file_writer(
        &mut self,
        w: FileWriter<<S as Store>::Writer>,
    ) -> IOResult<Link<<S as Store>::Key>> {
        self.commit_writer_kind(w.unwrap(), Kind::File)
    }

    pub fn put_dir(&mut self, d: &Dir<<S as Store>::Key>) -> IOResult<Link<<S as Store>::Key>> {
        use std::io::Write;

        let bytes = codec::encode_bytes(d);
        let mut w = self.0.open_writer()?;
        w.write_all(&bytes[..])?;
        self.commit_writer_kind(w, Kind::Dir)
    }

    pub fn open_entry_reader(&self, link: &Link<<S as Store>::Key>) -> IOResult<ReadEntry<S>> {
        let key = &link.key;
        match link.kind {
            Kind::Dir => self.get_dir(key).map(ReadEntry::Dir),
            Kind::File => self.0.open_reader(key).map(ReadEntry::FileStream),
        }
    }

    fn commit_writer_kind(
        &mut self,
        w: <S as Store>::Writer,
        kind: Kind,
    ) -> IOResult<Link<<S as Store>::Key>> {
        let key = self.0.commit_writer(w)?;
        Ok(Link { kind, key })
    }

    fn get_dir(&self, key: &<S as Store>::Key) -> IOResult<Dir<<S as Store>::Key>> {
        let bytes = self.0.read_bytes(key)?;
        let dir = codec::decode_bytes(&bytes[..]).map_err(|e| {
            use std::io::{Error, ErrorKind::InvalidData};

            Error::new(InvalidData, format!("Could not decode dir link: {:?}", e))
        })?;
        Ok(dir)
    }
}
