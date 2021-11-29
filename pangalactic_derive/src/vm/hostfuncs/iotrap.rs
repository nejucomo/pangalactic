#[derive(derive_more::From)]
pub(super) enum IOTrap {
    Trap(wasmi::Trap),
    Stdio(std::io::Error),
}

impl From<IOTrap> for wasmi::Trap {
    fn from(iot: IOTrap) -> wasmi::Trap {
        match iot {
            IOTrap::Trap(t) => t,
            IOTrap::Stdio(e) => pangalactic_wasmi::into_trap(e),
        }
    }
}
