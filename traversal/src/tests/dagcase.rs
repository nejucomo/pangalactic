use crate::TraversableDag;
use tokio_stream::Stream;

pub struct Case<D, V>
where
    D: TraversableDag,
    D::Error: std::error::Error + Send + Sync + 'static,
    V: PartialEq + From<D> + std::fmt::Debug + 'static,
{
    pub dag: D,
    pub children: &'static [V],
    pub bfs: &'static [V],
    pub dfs: &'static [V],
}

impl<D, V> Case<D, V>
where
    D: TraversableDag,
    D::Error: std::error::Error + Send + Sync + 'static,
    V: PartialEq + From<D> + std::fmt::Debug + 'static,
{
    pub async fn verify_children(self) -> anyhow::Result<()> {
        let stream = self.dag.children().await?;
        verify_stream(stream, self.children).await?;
        Ok(())
    }

    pub async fn verify_breadth_first_traversal(self) -> anyhow::Result<()> {
        verify_stream(self.dag.traverse_breadth_first(), self.bfs).await?;
        Ok(())
    }

    pub async fn verify_depth_first_traversal(self) -> anyhow::Result<()> {
        verify_stream(self.dag.traverse_depth_first(), self.dfs).await?;
        Ok(())
    }
}

async fn verify_stream<S, V, D>(stream: S, expected: &[V]) -> anyhow::Result<()>
where
    S: Stream<Item = Result<D, D::Error>>,
    V: PartialEq + From<D> + std::fmt::Debug + 'static,
    D: TraversableDag,
    D::Error: std::error::Error + Send + Sync + 'static,
{
    use tokio_stream::StreamExt;

    let actualres: Result<Vec<V>, D::Error> = stream.map(|res| res.map(V::from)).collect().await;
    let actual = actualres?;

    assert_eq!(actual, expected);
    Ok(())
}
