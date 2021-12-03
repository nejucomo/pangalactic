use crate::{Entry, Link};
use serde::{Deserialize, Serialize};
use std::io::{Result, Write};

// BUG: duplicate names
// BUG: canonicalization

#[derive(Debug, Deserialize, Serialize)]
pub struct Dir<K>(Vec<Entry<K>>);

impl<K> Dir<K> {
    pub fn new() -> Dir<K> {
        Dir(vec![])
    }

    pub fn add_link<S>(&mut self, name: S, link: Link<K>)
    where
        S: AsRef<str>,
    {
        let name = name.as_ref().to_string();
        self.push_entry(Entry { name, link });
    }

    pub fn push_entry(&mut self, entry: Entry<K>) {
        self.0.push(entry);
    }
}

impl<'a, K> IntoIterator for &'a Dir<K> {
    type Item = &'a Entry<K>;
    type IntoIter = std::slice::Iter<'a, Entry<K>>;

    fn into_iter(self) -> Self::IntoIter {
        (&self.0).into_iter()
    }
}

impl<K> Dir<K>
where
    K: serde::Serialize,
{
    pub fn to_user_json<W: Write>(&self, w: W) -> Result<()> {
        use pangalactic_codec::encode_string;
        use pangalactic_errorutil::io_error;
        use std::collections::HashMap;
        use std::io::ErrorKind::InvalidData;

        let mut hm = HashMap::new();
        for entry in self.0.iter() {
            let encodedlink = encode_string(&entry.link);
            if hm.insert(entry.name.clone(), encodedlink).is_some() {
                return Err(io_error!(
                    InvalidData,
                    "dir contains duplicate names for {:?}",
                    entry.name
                ));
            }
        }

        serde_json::to_writer_pretty(w, &hm)
            .map_err(|e| io_error!(InvalidData, "{:?} cannot serialize to JSON: {:?}", hm, e))
    }
}
