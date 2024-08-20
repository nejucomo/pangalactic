use pangalactic_guest::{define_derive, fail, log, unwrap, DirectoryWriter, Link, Plan};
use std::collections::BTreeMap;
use std::path::Path;

#[define_derive]
fn derive_impl(plan: Plan) -> Link {
    let mut dt = DirTree::new();

    for entres in unwrap!( Result tar::Archive::new(plan.input.open_file()).entries() ) {
        let entry = unwrap!( Result entres );
        let path = unwrap!( Result entry.path() ).into_owned();
        log!("unpacking entry path {:?}", &path);

        let link = pangalactic_guest::write_readable(entry);
        dt.insert(&path, link);
    }

    dt.commit()
}

struct DirTree(BTreeMap<String, Option<DTEntry>>);

enum DTEntry {
    File(Link),
    Sub(DirTree),
}

impl DirTree {
    fn new() -> Self {
        DirTree(BTreeMap::default())
    }

    fn insert(&mut self, path: &Path, link: Link) {
        let pc = path
            .components()
            .map(|comp| match comp {
                std::path::Component::Normal(s) => s.to_str().unwrap_or_else(|| {
                    fail!("tar path components that are non-utf8 such as {s:?} are unsupported")
                }),
                other => fail!("tar path components of {other:?} are unsupported"),
            })
            .peekable();

        self.insert_path_comp(pc, link);
    }

    fn insert_path_comp<'a, I>(&mut self, mut path: std::iter::Peekable<I>, link: Link)
    where
        I: Iterator<Item = &'a str>,
    {
        let comp = unwrap!( Option path.next() );
        let slot = self.0.entry(comp.to_string()).or_insert(None);
        slot_insert(slot, path, link);
    }

    fn commit(self) -> Link {
        let d = DirectoryWriter::open();
        for (name, optdte) in self.0.into_iter() {
            use DTEntry::*;

            match unwrap!( Option optdte ) {
                File(link) => d.insert(&name, link),
                Sub(sub) => d.insert(&name, sub.commit()),
            }
        }
        d.commit()
    }
}

fn slot_insert<'a, I>(slot: &mut Option<DTEntry>, mut path: std::iter::Peekable<I>, link: Link)
where
    I: Iterator<Item = &'a str>,
{
    use DTEntry::*;

    let newstate = match slot.take() {
        None => {
            if path.peek().is_some() {
                let mut dt = DirTree::new();
                dt.insert_path_comp(path, link);
                Sub(dt)
            } else {
                File(link)
            }
        }
        Some(Sub(mut sub)) => {
            if path.peek().is_some() {
                sub.insert_path_comp(path, link);
                Sub(sub)
            } else {
                fail!("Collision writing {link:?} to path with existing directory.");
            }
        }
        Some(File(other)) => {
            // TODO: `path.intersperse("/").collect::<String>()` requires nightly.
            let pathstr = path.collect::<Vec<&str>>().join("/");
            fail!("Collision overwriting file {other:?} with {link:?} at subpath {pathstr:?}",);
        }
    };

    *slot = Some(newstate);
}
