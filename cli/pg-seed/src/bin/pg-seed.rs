use anyhow::Result;
use pangalactic_cli_seed::SeedApplication;
use pangalactic_runopt::Application;

#[tokio::main(flavor = "current_thread")]
async fn main() -> Result<()> {
    SeedApplication::run_main().await
}
