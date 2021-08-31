use crate::{NodeStore, ReadEntry};
use pangalactic_node::Link;
use pangalactic_store::Store;

pub struct Traverse<'a, S>
where
    S: Store,
{
    store: &'a NodeStore<S>,
    linkq: Vec<Link<<S as Store>::Key>>,
}

impl<'a, S> Traverse<'a, S>
where
    S: Store,
{
    pub(crate) fn new(store: &'a NodeStore<S>, link: Link<<S as Store>::Key>) -> Self {
        Traverse {
            store,
            linkq: vec![link],
        }
    }
}

impl<'a, S> Iterator for Traverse<'a, S>
where
    S: Store,
{
    type Item = std::io::Result<ReadEntry<S>>;

    fn next(&mut self) -> Option<Self::Item> {
        if let Some(link) = self.linkq.pop() {
            let io = self.store.open_entry_reader(&link);
            match &io {
                Ok(ReadEntry::Dir(d)) => {
                    for entry in d.as_ref() {
                        self.linkq.push(entry.link.clone());
                    }
                }
                _ => {}
            }
            Some(io)
        } else {
            None
        }
    }
}
