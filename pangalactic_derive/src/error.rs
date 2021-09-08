use pangalactic_errorutil::into_std_error;

#[derive(Debug, derive_more::From)]
pub enum Error {
    IO(std::io::Error),
    WASM(wasmi::Error),
}

into_std_error!(Error, std::io::ErrorKind::Other);

pub type Result<T> = std::result::Result<T, Error>;
