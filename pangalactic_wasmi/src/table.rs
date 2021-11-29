/// A Table holds host values which can be referred to in the guest with Handles.
/// This design is only type safe against context confusion bugs if there is only one table for each type.
mod handle;

use wasmi::{Trap, TrapKind::TableAccessOutOfBounds};

pub struct Table<T>(Vec<T>);

pub use self::handle::Handle;

impl<T> Table<T> {
    pub fn new() -> Self {
        Table(vec![])
    }

    pub fn append(&mut self, item: T) -> Handle<T> {
        let h = Handle::from(self.0.len());
        self.0.push(item);
        h
    }

    pub fn get(&self, h: Handle<T>) -> Result<&T, Trap> {
        let ix = usize::from(h);
        if ix < self.0.len() {
            Ok(&self.0[ix])
        } else {
            Err(Trap::new(TableAccessOutOfBounds))
        }
    }

    pub fn get_mut(&mut self, h: Handle<T>) -> Result<&mut T, Trap> {
        let ix = usize::from(h);
        if ix < self.0.len() {
            Ok(&mut self.0[ix])
        } else {
            Err(Trap::new(TableAccessOutOfBounds))
        }
    }
}
