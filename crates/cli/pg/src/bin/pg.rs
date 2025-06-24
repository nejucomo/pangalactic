use anyhow::Result;
use pangalactic_cli::PgApplication;
use pangalactic_runopt::Application;

#[tokio::main(flavor = "current_thread")]
async fn main() -> Result<()> {
    PgApplication::run_main().await
}
