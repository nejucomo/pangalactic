#[derive(Debug)]
pub struct Builder<Key> {
    links: Vec<(String, Key)>,
}

impl<K> Builder<K> {
    pub fn new() -> Builder<K> {
        Builder { links: vec![] }
    }

    pub fn append_link(&mut self, name: String, key: K) {
        self.links.push((name, key));
    }

    pub fn finish_links(self) -> crate::DagNode<K> {
        crate::DagNode::from_links(self.links)
    }
}
