use serde::{Deserialize, Serialize};

#[derive(Debug, Deserialize, Serialize)]
pub struct DagNode<Key> {
    links: Vec<(String, Key)>,
    body: Vec<u8>,
}

impl<K> DagNode<K> {
    pub fn builder() -> crate::builder::Builder<K> {
        crate::builder::Builder::new()
    }

    pub(crate) fn from_links(links: Vec<(String, K)>) -> DagNode<K> {
        DagNode {
            links,
            body: vec![],
        }
    }
}

impl<K> std::io::Write for DagNode<K> {
    fn write(&mut self, buf: &[u8]) -> std::io::Result<usize> {
        self.body.write(buf)
    }

    fn flush(&mut self) -> std::io::Result<()> {
        self.body.flush()
    }
}
