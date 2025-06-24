use anyhow::Result;
use pangalactic_cli_store::StoreApplication;
use pangalactic_runopt::Application;

#[tokio::main(flavor = "current_thread")]
async fn main() -> Result<()> {
    StoreApplication::run_main().await
}
