#[derive(derive_more::From)]
pub(super) enum IOTrap {
    Trap(wasmi::Trap),
    Stdio(std::io::Error),
    Utf8(std::str::Utf8Error),
}

impl From<IOTrap> for wasmi::Trap {
    fn from(iot: IOTrap) -> wasmi::Trap {
        match iot {
            IOTrap::Trap(t) => t,
            IOTrap::Stdio(e) => pangalactic_wasmi::into_trap(e),
            IOTrap::Utf8(e) => pangalactic_wasmi::into_trap(e),
        }
    }
}
