use pangalactic_name::NameRef;

use crate::{bindings, prim, Link};

#[derive(Debug)]
pub struct DirectoryWriter(prim::HandleDirWriter);

impl DirectoryWriter {
    pub fn open() -> Self {
        DirectoryWriter(unsafe { bindings::directory_writer_open() })
    }

    pub fn insert(&self, name: &NameRef, link: Link) {
        unsafe {
            let (ptr, len) = crate::ptr::unpack_for_write(name.as_bytes());
            bindings::directory_writer_insert(self.0, ptr, len, link.unwrap_handle());
        }
    }

    pub fn commit(self) -> Link {
        unsafe { Link::wrap_handle(bindings::directory_writer_commit(self.0)) }
    }
}
