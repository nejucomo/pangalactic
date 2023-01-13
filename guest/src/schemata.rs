use crate::Link;

pub type Attestation = dagwasm_schemata::Attestation<Link>;
pub type Directory = dagwasm_dir::Directory<Link>;
pub type Plan = dagwasm_schemata::Plan<Link>;

impl From<Link> for Directory {
    fn from(link: Link) -> Self {
        dagwasm_dir::Directory::from_iter(link.open_directory())
    }
}

impl From<Directory> for Link {
    fn from(dir: Directory) -> Self {
        let dw = crate::DirectoryWriter::open();
        for (n, l) in dir {
            dw.insert(&n, l);
        }
        dw.commit()
    }
}

impl From<Link> for Plan {
    fn from(link: Link) -> Self {
        let d = Directory::from(link);
        Plan::try_from(d).unwrap()
    }
}

impl From<Plan> for Link {
    fn from(plan: Plan) -> Link {
        Link::from(Directory::from(plan))
    }
}

impl From<Link> for Attestation {
    fn from(link: Link) -> Self {
        let d = Directory::from(link);
        Attestation::try_from(d).unwrap()
    }
}

impl From<Attestation> for Link {
    fn from(att: Attestation) -> Link {
        Link::from(Directory::from(att))
    }
}
