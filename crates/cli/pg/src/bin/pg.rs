use anyhow::Result;
use pangalactic_cli::PgOptions;
use pangalactic_runopt::Application;

#[tokio::main(flavor = "current_thread")]
async fn main() -> Result<()> {
    PgOptions::run_main().await
}
