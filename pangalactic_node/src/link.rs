use serde::{Deserialize, Serialize};

#[derive(Debug, Deserialize, Serialize)]
pub struct Link<Key> {
    kind: Kind,
    key: Key,
}

#[derive(Debug, Deserialize, Serialize)]
pub enum Kind {
    File,
    Dir,
}
