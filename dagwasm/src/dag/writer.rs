use async_trait::async_trait;

#[async_trait]
pub trait Writer: tokio::io::AsyncWrite {
    type Link;

    async fn commit(self) -> anyhow::Result<Self::Link>;
}
