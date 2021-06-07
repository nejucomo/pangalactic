#[derive(Debug, derive_more::From)]
pub enum Error {
    Io(std::io::Error),
    Wasmi(wasmi::Error),
    TokioJoin(tokio::task::JoinError),
}
