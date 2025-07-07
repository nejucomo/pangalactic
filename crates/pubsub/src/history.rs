use serde::{Deserialize, Serialize};

#[derive(Debug, Serialize, Deserialize)]
pub enum History<L> {
    Initial { content: L },
    Subsequent { content: L, prev: L },
}
