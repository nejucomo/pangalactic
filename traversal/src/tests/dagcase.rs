use crate::TraversableDag;
use std::fmt::Debug;
use tokio_stream::Stream;

pub struct Case<D>
where
    D: DagCase,
    <D as TraversableDag>::Error: Send + Sync + 'static,
{
    pub dag: D::Ctr,
    pub children: &'static [D::Verifier],
    pub bfs: &'static [D::Verifier],
    pub dfs: &'static [D::Verifier],
}

pub trait DagCase: Clone + TraversableDag<Error = anyhow::Error> {
    type Ctr;
    type Verifier: From<Self> + PartialEq + Debug + 'static;

    async fn setup(constructor: Self::Ctr) -> anyhow::Result<Self>;
    async fn cleanup(self) -> anyhow::Result<()>;
}

impl<D> Case<D>
where
    D: DagCase,
    <D as TraversableDag>::Error: Send + Sync + 'static,
{
    pub async fn verify_children(self) -> anyhow::Result<()> {
        let dag = D::setup(self.dag).await?;
        let stream = dag.children().await?;
        verify_stream(stream, self.children).await?;
        dag.cleanup().await?;
        Ok(())
    }

    pub async fn verify_breadth_first_traversal(self) -> anyhow::Result<()> {
        let dag = D::setup(self.dag).await?;
        verify_stream(dag.clone().traverse_breadth_first(), self.bfs).await?;
        dag.cleanup().await?;
        Ok(())
    }

    pub async fn verify_depth_first_traversal(self) -> anyhow::Result<()> {
        let dag = D::setup(self.dag).await?;
        verify_stream(dag.clone().traverse_depth_first(), self.dfs).await?;
        dag.cleanup().await?;
        Ok(())
    }
}

async fn verify_stream<S, D>(stream: S, expected: &[D::Verifier]) -> anyhow::Result<()>
where
    S: Stream<Item = anyhow::Result<D>>,
    D: DagCase,
    <D as TraversableDag>::Error: Send + Sync + 'static,
{
    use tokio_stream::StreamExt;

    let actualres: Result<Vec<D::Verifier>, D::Error> =
        stream.map(|res| res.map(D::Verifier::from)).collect().await;
    let actual = actualres?;

    assert_eq!(actual, expected);
    Ok(())
}
