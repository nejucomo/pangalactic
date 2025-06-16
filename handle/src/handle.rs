use std::fmt;
use std::marker::PhantomData;

#[derive(Clone)]
pub struct Handle<T> {
    ix: u64,
    phantom_item: PhantomData<T>,
}

impl<T> Handle<T> {
    /// Wrap a bare `u64` into a `Handle`.
    ///
    /// # Safety
    ///
    /// Only code which authoritatively creates and dereferences handles should use this method,
    /// such as container type indexed by `Handle<T>` or a host. In particular this should never
    /// be called on arbitrary values, especially from untrusted sources, such as dag contents.
    pub unsafe fn wrap(ix: u64) -> Self {
        Handle {
            ix,
            phantom_item: PhantomData,
        }
    }

    /// Peek at the bare `u64` wrapped by this `Handle`.
    ///
    /// # Safety
    ///
    /// Only the same "authority" which created a handle should ever peek at this value.
    pub unsafe fn peek(&self) -> u64 {
        self.ix
    }
}

impl<T> fmt::Debug for Handle<T> {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "{}({})", std::any::type_name::<Self>(), self.ix)
    }
}
