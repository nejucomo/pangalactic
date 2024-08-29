use std::future::Future;

use anyhow::Result;

use crate::IntoSource;

#[derive(Debug)]
pub enum NSource<L, B> {
    Leaf(L),
    Branch(B),
}

impl<L, B> IntoSource<L, B> for NSource<L, B>
where
    L: Send,
    B: Send,
{
    fn into_source<S>(
        self,
        _: &pangalactic_layer_dir::LinkDirectoryLayer<S>,
    ) -> impl Future<Output = Result<NSource<L, B>>> + Send
    where
        S: pangalactic_store::Store,
    {
        std::future::ready(Ok(self))
    }
}
