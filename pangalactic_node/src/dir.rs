use crate::Entry;
use serde::{Deserialize, Serialize};

#[derive(Debug, Deserialize, Serialize)]
pub struct Dir<K>(Vec<Entry<K>>);

impl<K> Dir<K> {
    pub fn new() -> Dir<K> {
        Dir(vec![])
    }

    pub fn push_entry(&mut self, entry: Entry<K>) {
        self.0.push(entry);
    }
}
