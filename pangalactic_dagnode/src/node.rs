use serde::{Deserialize, Serialize};

#[derive(Debug, Deserialize, Serialize)]
pub struct DagNode<Key> {
    links: Vec<(String, Key)>,
    body: Vec<u8>,
}

impl<K> DagNode<K> {
    pub fn new() -> DagNode<K> {
        DagNode {
            links: vec![],
            body: vec![],
        }
    }

    pub fn append_link(&mut self, name: String, key: K) {
        self.links.push((name, key));
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
