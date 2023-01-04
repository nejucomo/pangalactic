use crate::prim::{
    Bool, ByteLen, HandleByteReader, HandleDirReader, HandleLink, LinkKind, PtrRead,
};

#[link(wasm_import_module = "dagwasm-host")]
extern "C" {
    // Link methods:
    pub fn link_get_kind(handle_link: HandleLink) -> LinkKind;
    pub fn link_open_file_reader(handle_link: HandleLink) -> HandleByteReader;
    pub fn link_open_directory_reader(handle_link: HandleLink) -> HandleDirReader;

    // ByteReader methods:
    pub fn byte_reader_read(
        handle_byte_reader: HandleByteReader,
        ptr: PtrRead,
        len: ByteLen,
    ) -> ByteLen;
    pub fn byte_reader_close(handle_byte_reader: HandleByteReader);

    // DirectoryReader methods:
    pub fn directory_reader_has_more_entries(handle_directory_reader: HandleDirReader) -> Bool;
    pub fn directory_reader_open_name_reader(
        handle_directory_reader: HandleDirReader,
    ) -> HandleByteReader;
    pub fn directory_reader_load_link(handle_directory_reader: HandleDirReader) -> HandleLink;
    pub fn directory_reader_next_entry(handle_directory_reader: HandleDirReader);
    pub fn directory_reader_close(handle_byte_reader: HandleByteReader);
}
