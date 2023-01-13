use crate::Link;

pub type Directory = dagwasm_dir::Directory<Link>;

impl From<Link> for Directory {
    fn from(link: Link) -> Self {
        dagwasm_dir::Directory::from_iter(link.open_directory())
    }
}

impl From<Directory> for Link {
    fn from(dir: Directory) -> Self {
        use crate::DirectoryWriter;

        let dw = DirectoryWriter::open();
        for (n, l) in dir {
            dw.insert(&n, l);
        }
        dw.commit()
    }
}
