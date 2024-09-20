use std::future::Future;

use anyhow::Result;

pub trait RunOptions<T> {
    fn run_options(&self, options: &T) -> impl Future<Output = Result<()>> + Send;
}
