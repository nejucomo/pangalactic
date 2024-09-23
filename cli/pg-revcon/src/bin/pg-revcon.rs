use anyhow::Result;
use pangalactic_cli_revcon::RevConApplication;
use pangalactic_runopt::Application;

#[tokio::main(flavor = "current_thread")]
async fn main() -> Result<()> {
    RevConApplication::run_main().await
}
