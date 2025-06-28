use anyhow::Result;
use pangalactic_cli_revcon::Options;
use pangalactic_runopt::Application;

#[tokio::main(flavor = "current_thread")]
async fn main() -> Result<()> {
    Options::run_main().await
}
