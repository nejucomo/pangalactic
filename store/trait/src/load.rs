use crate::Store;

pub trait Load<S>: Sized
where
    S: Store,
{
    async fn load_from_store(store: &S, cid: &S::CID) -> anyhow::Result<Self>;
}

// impl<S> Load<S> for S::CID
// where
//     S: Store,
//     S::CID: Clone,
// {
//     async fn load_from_store(_: &S, cid: &S::CID) -> anyhow::Result<Self> {
//         Ok(cid.clone())
//     }

impl<S> Load<S> for Vec<u8>
where
    S: Store,
{
    async fn load_from_store(store: &S, cid: &S::CID) -> anyhow::Result<Self> {
        use tokio::io::AsyncReadExt;

        let mut r: S::Reader = store.load(cid).await?;
        let mut buf = vec![];
        r.read_to_end(&mut buf).await?;
        Ok(buf)
    }
}
