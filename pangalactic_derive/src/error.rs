#[derive(Debug, derive_more::From)]
pub enum Error {
    IO(std::io::Error),
    WASM(wasmi::Error),
}

pub type Result<T> = std::result::Result<T, Error>;
