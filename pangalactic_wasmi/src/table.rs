/// A Table holds host values which can be referred to in the guest with Handles.
/// This design is only type safe against context confusion bugs if there is only one table for each type.
mod handle;

use wasmi::{
    Trap,
    TrapKind::{ElemUninitialized, TableAccessOutOfBounds},
};

pub struct Table<T>(Vec<Option<T>>);

pub use self::handle::Handle;

impl<T> Table<T> {
    pub fn new() -> Self {
        Table(vec![])
    }

    pub fn insert(&mut self, item: T) -> Handle<T> {
        for (ix, slot) in self.0.iter_mut().enumerate() {
            if slot.is_none() {
                slot.replace(item);
                return Handle::from(ix);
            }
        }

        // No tombstones found, so allocate a new slot:
        let h = Handle::from(self.0.len());
        self.0.push(Some(item));
        h
    }

    pub fn release(&mut self, h: Handle<T>) -> Result<(), Trap> {
        let slot = self.get_slot_mut(h)?;
        if slot.is_none() {
            Err(Trap::new(ElemUninitialized))
        } else {
            *slot = None;
            Ok(())
        }
    }

    pub fn get(&self, h: Handle<T>) -> Result<&T, Trap> {
        deref_slot(self.get_slot(h)?.as_ref())
    }

    pub fn get_mut(&mut self, h: Handle<T>) -> Result<&mut T, Trap> {
        deref_slot(self.get_slot_mut(h)?.as_mut())
    }

    fn get_slot(&self, h: Handle<T>) -> Result<&Option<T>, Trap> {
        deref_slot(self.0.get(usize::from(h)))
    }

    fn get_slot_mut(&mut self, h: Handle<T>) -> Result<&mut Option<T>, Trap> {
        deref_slot(self.0.get_mut(usize::from(h)))
    }
}

fn deref_slot<T>(slot: Option<T>) -> Result<T, Trap> {
    slot.ok_or(Trap::new(TableAccessOutOfBounds))
}
