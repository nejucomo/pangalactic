#[derive(Debug)]
pub enum Error {
    IO(std::io::Error),
    WASMI(wasmi::Error),
}

// FIXME: use derive_more::From
impl From<std::io::Error> for Error {
    fn from(e: std::io::Error) -> Error {
        Error::IO(e)
    }
}

impl From<wasmi::Error> for Error {
    fn from(e: wasmi::Error) -> Error {
        Error::WASMI(e)
    }
}
