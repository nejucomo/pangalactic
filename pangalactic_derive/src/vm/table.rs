/// A Table holds host values which can be referred to in the guest with Handles.
/// This design is only type safe against context confusion bugs if there is only one table for each type.
mod handle;

pub struct Table<T>(Vec<T>);

pub use self::handle::Handle;

impl<T> Table<T> {
    pub(crate) fn new() -> Self {
        Table(vec![])
    }

    pub(crate) fn append(&mut self, item: T) -> Handle<T> {
        let h = Handle::from(self.0.len());
        self.0.push(item);
        h
    }
}

impl<T> std::ops::Index<Handle<T>> for Table<T> {
    type Output = T;

    fn index(&self, index: Handle<T>) -> &T {
        self.0.index(usize::from(index))
    }
}
