use crate::{bindings, prim, ByteReader, DirectoryReader, Reader};
use dagwasm_linkkind::LinkKind;

#[derive(Debug)]
pub struct Link(prim::HandleLink);

impl Link {
    pub fn kind(&self) -> LinkKind {
        let lkprim: prim::LinkKind = unsafe { bindings::link_get_kind(self.0) };
        LinkKind::try_from(lkprim).unwrap()
    }

    pub fn open(&self) -> Reader {
        use LinkKind::*;

        let reader = match self.kind() {
            File => Reader::File(ByteReader::wrap_handle(
                unsafe { bindings::link_open_file_reader(self.0) },
                true,
            )),
            Dir => Reader::Dir(DirectoryReader::wrap_handle(unsafe {
                bindings::link_open_directory_reader(self.0)
            })),
        };

        trace!("{:?}.open() -> {:?}", self, &reader);
        reader
    }

    pub fn open_file(&self) -> ByteReader {
        use Reader::*;

        if let File(reader) = self.open() {
            reader
        } else {
            fail!("expected {:?} to be a file", self)
        }
    }

    pub fn open_directory(&self) -> DirectoryReader {
        use Reader::*;

        if let Dir(reader) = self.open() {
            reader
        } else {
            fail!("expected {:?} to be a directory", self)
        }
    }

    /// Wrap a bare primitive handle from the host.
    ///
    /// # Safety
    ///
    /// Only to be used by this crate _or_ the `derive` entrypoint wrapper code. User code should
    /// never call this directly, thus `doc(hidden)`.
    #[doc(hidden)]
    pub unsafe fn wrap_handle(h: prim::HandleLink) -> Self {
        Link(h)
    }

    /// Unwrap a primitive handle, giving the caller full responsibility for the handle.
    ///
    /// # Safety
    ///
    /// Only to be used by this crate _or_ the `derive` exitpoint wrapper code. User code should
    /// never call this directly, thus `doc(hidden)`. The caller is responsible for cleaning up the
    /// handle if necessary.
    #[doc(hidden)]
    pub unsafe fn unwrap_handle(self) -> prim::HandleLink {
        let h = self.0;
        // Do not exercise drop/close because the caller is responsible for the handle.
        std::mem::forget(self);
        h
    }
}

impl std::ops::Drop for Link {
    fn drop(&mut self) {
        unsafe { bindings::link_close(self.0) };
    }
}
