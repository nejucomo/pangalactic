// mod json;

use crate::Entry;
use serde::{Deserialize, Serialize};
use std::io::{Result, Write};

#[derive(Debug, Deserialize, Serialize)]
pub struct Dir<K>(Vec<Entry<K>>);

impl<K> Dir<K> {
    pub fn new() -> Dir<K> {
        Dir(vec![])
    }

    pub fn push_entry(&mut self, entry: Entry<K>) {
        self.0.push(entry);
    }

    pub fn to_user_json<W: Write>(&mut self, _w: W) -> Result<()> {
        todo!("to_user_json(...)");
        /*
        use pangalactic_errorutil::io_error;
        use std::io::ErrorKind::InvalidData;

        Dir(d) => serde_json::to_writer_pretty(out, &d)
            .map_err(|e| io_error!(InvalidData, "JSON serialization failure: {:#?}", e)),
        */
    }
}
