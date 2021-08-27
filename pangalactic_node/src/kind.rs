use serde::{Deserialize, Serialize};

#[derive(Debug, Eq, PartialEq, Deserialize, Serialize)]
pub enum Kind {
    File,
    Dir,
}
