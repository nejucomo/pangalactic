use anyhow::Result;
use pangalactic_cli_derive::DeriveApplication;
use pangalactic_runopt::Application;

#[tokio::main(flavor = "current_thread")]
async fn main() -> Result<()> {
    DeriveApplication::run_main().await
}
