#[derive(Debug, derive_more::From)]
pub enum Error {
    IO(std::io::Error),
    WASMI(wasmi::Error),
}
