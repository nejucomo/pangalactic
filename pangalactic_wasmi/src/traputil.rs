use std::fmt::{self, Debug, Display};
use wasmi::{HostError, Trap};

pub fn into_trap<E>(error: E) -> Trap
where
    E: 'static + Send + Sync + Debug,
{
    use wasmi::TrapKind::Host;

    Trap::new(Host(Box::new(TrapWrapper(error))))
}

/// Wrap any type meeting constraints so that it's usable as a [`TrapKind::Host`].
#[derive(Debug)]
struct TrapWrapper<T>(T)
where
    T: 'static + Send + Sync + Debug;

impl<T> Display for TrapWrapper<T>
where
    T: 'static + Send + Sync + Debug,
{
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "{}({:?})", std::any::type_name::<Self>(), &self.0)
    }
}

impl<T> HostError for TrapWrapper<T> where T: 'static + Send + Sync + Debug {}
