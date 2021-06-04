#![deny(warnings)]

#[macro_use]
mod trace;

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

use ipfs_api::IpfsClient;

async fn test_ipfs() -> Result<(), Error> {
    let client = IpfsClient::default();
    let cid = ipfs_put_then_get(&client, r#"{ "Hello": "World" }"#).await?;
    let dag2 = trace!(format!(r#"{{ "wut": "{}" }}"#, &cid));
    ipfs_put_then_get(&client, &dag2).await?;

    Ok(())
}

async fn ipfs_put_then_get(client: &IpfsClient, s: &str) -> Result<String, Error> {
    use futures::TryStreamExt;
    let mys = String::from(s);
    let cid = trace!(client.dag_put(std::io::Cursor::new(mys)).await)?
        .cid
        .cid_string;

    let v = trace!(
        client
            .dag_get(&cid)
            .map_ok(|chunk| chunk.to_vec())
            .try_concat()
            .await
    )?;

    trace!(String::from_utf8_lossy(&v[..]));
    Ok(cid)
}
