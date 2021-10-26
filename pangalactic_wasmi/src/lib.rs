#![feature(fn_traits)]

mod args;
mod hostfunc;
mod resolver;
mod ret;
mod table;
mod value;

pub use self::args::FromGuestArgs;
pub(crate) use self::hostfunc::HostFn1;
pub use self::hostfunc::HostFunc;
pub use self::resolver::HostFuncResolver;
pub use self::ret::IntoGuestReturn;
pub use self::table::{Handle, Table};
pub use self::value::{FromGuestValue, IntoGuestValue};
