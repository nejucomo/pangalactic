#![feature(fn_traits)]

mod args;
mod hostfunc;
mod resolver;
mod ret;
mod table;
mod traputil;
mod value;

pub use self::args::FromGuestArgs;
pub use self::hostfunc::HostFunc;
pub(crate) use self::hostfunc::{HostFn0, HostFn1, HostFn2, HostFn3};
pub use self::resolver::HostFuncResolver;
pub use self::ret::{IntoGuestReturn, Void};
pub use self::table::{Handle, Table};
pub use self::traputil::into_trap;
pub use self::value::{FromGuestValue, IntoGuestValue};
