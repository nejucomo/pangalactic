use crate::prim::{
    Bool, ByteLen, HandleByteReader, HandleByteWriter, HandleDirReader, HandleDirWriter,
    HandleLink, LinkKind, PtrRead, PtrWrite,
};

#[link(wasm_import_module = "dagwasm-host")]
extern "C" {
    // Log:
    pub fn log(ptr: PtrWrite, len: ByteLen);

    // Link methods:
    pub fn link_get_kind(link: HandleLink) -> LinkKind;
    pub fn link_open_file_reader(link: HandleLink) -> HandleByteReader;
    pub fn link_open_directory_reader(link: HandleLink) -> HandleDirReader;
    pub fn link_close(link: HandleLink);

    // ByteReader methods:
    pub fn byte_reader_read(byte_reader: HandleByteReader, ptr: PtrRead, len: ByteLen) -> ByteLen;
    pub fn byte_reader_close(byte_reader: HandleByteReader);

    // DirectoryReader methods:
    pub fn directory_reader_has_more_entries(directory_reader: HandleDirReader) -> Bool;
    pub fn directory_reader_open_name_reader(directory_reader: HandleDirReader)
        -> HandleByteReader;
    pub fn directory_reader_load_link(directory_reader: HandleDirReader) -> HandleLink;
    pub fn directory_reader_next_entry(directory_reader: HandleDirReader);
    pub fn directory_reader_close(byte_reader: HandleByteReader);

    // ByteWriter methods:
    pub fn byte_writer_open() -> HandleByteWriter;
    pub fn byte_writer_write(byte_writer: HandleByteWriter, ptr: PtrWrite, len: ByteLen);
    pub fn byte_writer_commit(byte_writer: HandleByteWriter) -> HandleLink;

    // DirectoryWriter methods:
    pub fn directory_writer_open() -> HandleDirWriter;
    pub fn directory_writer_insert(
        directory_writer: HandleDirWriter,
        nameptr: PtrWrite,
        namelen: ByteLen,
        link: HandleLink,
    );
    pub fn directory_writer_commit(directory_writer: HandleDirWriter) -> HandleLink;
}
