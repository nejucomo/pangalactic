#[tokio::main(flavor = "current_thread")]
async fn main() -> anyhow::Result<()> {
    pangalactic_cli::run().await
}
