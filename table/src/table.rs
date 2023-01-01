use dagwasm_handle::Handle;
use std::collections::BTreeMap;

#[derive(Debug)]
pub struct Table<T> {
    nextix: u64,
    map: BTreeMap<u64, T>,
}

impl<T> Default for Table<T> {
    fn default() -> Self {
        Table {
            nextix: 0,
            map: BTreeMap::default(),
        }
    }
}

impl<T> Table<T> {
    pub fn insert(&mut self, item: T) -> Handle<T> {
        assert!(self.nextix < u64::MAX);
        let ix = self.nextix;
        self.nextix += 1;
        assert!(self.map.insert(ix, item).is_none());
        unsafe { Handle::wrap(ix) }
    }

    pub fn lookup(&self, handle: Handle<T>) -> anyhow::Result<&T> {
        self.map
            .get(&unsafe { handle.peek() })
            .ok_or_else(|| anyhow::Error::msg(format!("invalid lookup {handle:?}")))
    }

    pub fn lookup_mut(&mut self, handle: Handle<T>) -> anyhow::Result<&mut T> {
        self.map
            .get_mut(&unsafe { handle.peek() })
            .ok_or_else(|| anyhow::Error::msg(format!("invalid lookup {handle:?}")))
    }
}
