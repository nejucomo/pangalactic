#![deny(warnings)]

mod cli;

use log::{debug, info};
use pangalactic;
use std::path::PathBuf;
use tokio;

#[derive(Debug, derive_more::From)]
enum Error {
    Cli(cli::Error),
    Logger(log::SetLoggerError),
    IpfsResponse(ipfs_api::response::Error),
    Pangalactic(pangalactic::Error),
}

#[tokio::main]
async fn main() -> Result<(), Error> {
    use cli::Command;

    simple_logger::SimpleLogger::new().init()?;

    let cmd = Command::parse_args(std::env::args().skip(1))?;
    debug!("cli: {:?}", &cmd);
    match cmd {
        Command::TestWasm(path) => test_wasm_path(path),
        Command::TestIpfs => test_ipfs().await,
    }
}

fn test_wasm_path(guestpath: PathBuf) -> Result<(), Error> {
    info!("=== {} test wasm ===", env!("CARGO_PKG_NAME"));
    pangalactic::execute_path(guestpath)?;
    Ok(())
}

async fn test_ipfs() -> Result<(), Error> {
    use ipfs_api::IpfsClient;

    let client = IpfsClient::default();

    dbg!(
        client
            .dag_put(std::io::Cursor::new(r#"{ "Hello": "World" }"#))
            .await?
    );
    Ok(())
}
