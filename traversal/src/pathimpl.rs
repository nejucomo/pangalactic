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
            Box::pin(pathbuf_dir_to_future(self.clone()))
        } else {
            let stream: ChildStream = Box::pin(tokio_stream::empty());

            Box::pin(std::future::ready(Ok(stream)))
        }
    }
}

fn pathbuf_dir_to_future(p: PathBuf) -> impl Future<Output = IOResult<ChildStream>> + 'static {
    use futures::FutureExt;
    use tokio::fs::read_dir;

    read_dir(p.clone()).map(move |rdres| {
        rdres
            .map(read_dir_to_pathbuf_stream)
            .map_err(|e| anyhow::Error::from(e).context(format!("for path {:?}", p.display())))
    })
}

fn read_dir_to_pathbuf_stream(rd: tokio::fs::ReadDir) -> ChildStream {
    use tokio_stream::wrappers::ReadDirStream;
    use tokio_stream::StreamExt;

    Box::pin(ReadDirStream::new(rd).map(|entry_res| {
        entry_res
            .map(|entry| entry.path())
            .map_err(anyhow::Error::from)
    }))
}
