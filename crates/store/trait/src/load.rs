use std::future::Future;

use anyhow::Result;

use crate::Store;

pub trait Load<S>: Sized
where
    S: Store,
{
    fn load_from_store(store: &S, cid: &S::CID) -> impl Future<Output = Result<Self>> + Send;
}

impl<S> Load<S> for Vec<u8>
where
    S: Store,
{
    fn load_from_store(store: &S, cid: &S::CID) -> impl Future<Output = Result<Self>> + Send {
        use tokio::io::AsyncReadExt;

        async {
            let mut r: S::Reader = store.load(cid).await?;
            let mut buf = vec![];
            r.read_to_end(&mut buf).await?;
            Ok(buf)
        }
    }
}
