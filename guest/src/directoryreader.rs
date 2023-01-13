use crate::{bindings, prim, ByteReader, Link};

#[derive(Debug)]
pub struct DirectoryReader(prim::HandleDirReader);

impl DirectoryReader {
    pub(crate) fn wrap_handle(handle: prim::HandleDirReader) -> Self {
        DirectoryReader(handle)
    }

    fn next_inner(&self) -> Option<(String, Link)> {
        if unsafe { bindings::directory_reader_has_more_entries(self.0) } == prim::TRUE {
            let namereader = ByteReader::wrap_handle(
                unsafe { bindings::directory_reader_open_name_reader(self.0) },
                false,
            );
            let name = String::from_utf8(namereader.read_to_vec())
                .expect("invalid utf8 in directory entry name");
            let link = unsafe { Link::wrap_handle(bindings::directory_reader_load_link(self.0)) };
            unsafe { bindings::directory_reader_next_entry(self.0) };

            Some((name, link))
        } else {
            None
        }
    }
}

impl Iterator for DirectoryReader {
    type Item = (String, Link);

    fn next(&mut self) -> Option<Self::Item> {
        let item = self.next_inner();
        trace!("{self:?}.iter() -> {:?}", &item);
        item
    }
}

impl std::ops::Drop for DirectoryReader {
    fn drop(&mut self) {
        unsafe { bindings::directory_reader_close(self.0) };
    }
}
