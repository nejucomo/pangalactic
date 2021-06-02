#![deny(warnings)]

mod cli;

use log::{debug, info};
use pangalactic;
use tokio;

#[derive(Debug, derive_more::From)]
enum Error {
    Cli(cli::Error),
    Logger(log::SetLoggerError),
    Pangalactic(pangalactic::Error),
}

#[tokio::main]
async fn main() -> Result<(), Error> {
    simple_logger::SimpleLogger::new().init()?;

    let cmd = cli::Command::parse_args(std::env::args().skip(1))?;
    debug!("Parsed command: {:?}", cmd);

    // FIXME: test wasm:
    info!("=== test wasm {} ===", env!("CARGO_PKG_NAME"));
    let mut args = std::env::args().skip(1);
    let guestpath = args.next().unwrap();
    assert_eq!(None, args.next());

    pangalactic::execute_path(guestpath)?;

    Ok(())
}
