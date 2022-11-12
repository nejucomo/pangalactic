use async_trait::async_trait;
use std::fmt::Debug;
use tokio::io::AsyncWrite;

#[async_trait]
pub trait Writer: AsyncWrite {
    type Key: Debug;

    async fn commit(self) -> anyhow::Result<Self::Key>;
}
