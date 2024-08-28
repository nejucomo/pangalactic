use std::future::Future;

use anyhow::Result;

use crate::Source;

pub trait IntoSource {
    type Source: Source;

    fn into_source(self) -> impl Future<Output = Result<Self::Source>> + Send;
}
