mod fromguest;
mod intoguest;

use wasmi::Trap;

pub use self::fromguest::FromGuestValue;
pub use self::intoguest::IntoGuestValue;

pub(crate) fn invalid_int() -> Trap {
    Trap::new(wasmi::TrapKind::InvalidConversionToInt)
}
