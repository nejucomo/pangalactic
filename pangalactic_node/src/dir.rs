use serde::{Deserialize, Serialize};

#[derive(Debug, Deserialize, Serialize)]
pub struct Dir<K>(Vec<Entry<K>>);

#[derive(Debug, Deserialize, Serialize)]
pub struct Entry<K> {
    name: String,
    link: K,
}

impl<K> Dir<K> {
    pub fn new() -> Dir<K> {
        Dir(vec![])
    }

    pub fn append_link(&mut self, name: String, link: K) {
        self.0.push(Entry { name, link });
    }
}
