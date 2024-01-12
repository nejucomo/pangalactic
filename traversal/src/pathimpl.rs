use crate::TraversableDag;
use futures::Future;
use std::{path::PathBuf, pin::Pin};
use tokio_stream::Stream;

// type IOResult<T> = std::io::Result<T>;
type IOResult<T> = anyhow::Result<T>;

type ChildStream = Pin<Box<dyn Stream<Item = IOResult<PathBuf>> + 'static>>;

impl TraversableDag for PathBuf {
    // type Error = std::io::Error;
    type Error = anyhow::Error;
    type ChildStream = ChildStream;
    type ChildrenFut = Pin<Box<dyn Future<Output = IOResult<Self::ChildStream>>>>;

    fn children(&self) -> Self::ChildrenFut {
        if self.is_dir() {
            Box::pin(get_sorted_children(self.clone()))
        } else {
            // A non-directory has no children:
            let stream: ChildStream = Box::pin(tokio_stream::empty());
            Box::pin(std::future::ready(Ok(stream)))
        }
    }
}

async fn get_sorted_children(path: PathBuf) -> anyhow::Result<ChildStream> {
    use anyhow::Context;

    let ctx = format!("with path {:?}", path.display());
    get_sorted_children_without_context(path).await.context(ctx)
}

async fn get_sorted_children_without_context(path: PathBuf) -> anyhow::Result<ChildStream> {
    use tokio_stream::wrappers::ReadDirStream;
    use tokio_stream::StreamExt;

    let rd = tokio::fs::read_dir(path).await?;
    let res: anyhow::Result<Vec<PathBuf>> = ReadDirStream::new(rd)
        .map(|res| res.map_err(anyhow::Error::from))
        .map(|entres| entres.map(|entry| entry.path()))
        .collect()
        .await;
    let mut v = res?;
    v.sort();
    Ok(Box::pin(tokio_stream::iter(v.into_iter().map(Ok))))
}
